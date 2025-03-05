#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResultWithPostInfo,
        pallet_prelude::*,
        traits::{Currency, ReservableCurrency, Get, ExistenceRequirement},
        Parameter, 
        sp_runtime::{
            traits::{AtLeast32BitUnsigned, CheckedDiv, CheckedMul, MaybeSerializeDeserialize, Member, Zero},
            FixedPointNumber, FixedU128, Perquintill,
        },
    };
    use frame_system::pallet_prelude::*;
    use sp_std::prelude::*;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        
        /// The currency type this pallet will be using
        type Currency: ReservableCurrency<Self::AccountId>;
        
        /// Asset ID type
        type AssetId: Parameter + Member + Copy + MaybeSerializeDeserialize + Ord + Default + AtLeast32BitUnsigned;
        
        /// The custom fee applied to all trades (0.369%)
        #[pallet::constant]
        type TradingFee: Get<Perquintill>;
        
        /// The minimum liquidity required for a new pool
        #[pallet::constant]
        type MinimumLiquidity: Get<BalanceOf<Self>>;
    }

    pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    
    /// Nourish Chain Trading Fee set to 0.369%
    pub const NOURISH_CHAIN_FEE: Perquintill = Perquintill::from_parts(3_690_000_000_000_000); // 0.369%

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /// Storage for liquidity pools. Maps (asset_a, asset_b) -> (reserve_a, reserve_b)
    #[pallet::storage]
    #[pallet::getter(fn liquidity_pools)]
    pub type LiquidityPools<T: Config> = StorageMap<
        _, 
        Blake2_128Concat, 
        (T::AssetId, T::AssetId), 
        (BalanceOf<T>, BalanceOf<T>), 
        ValueQuery
    >;

    /// Storage for liquidity tokens. Maps (asset_a, asset_b, account) -> liquidity_tokens
    #[pallet::storage]
    #[pallet::getter(fn liquidity_tokens)]
    pub type LiquidityTokens<T: Config> = StorageMap<
        _, 
        Blake2_128Concat, 
        (T::AssetId, T::AssetId, T::AccountId), 
        BalanceOf<T>, 
        ValueQuery
    >;

    /// Total supply of liquidity tokens for each pool
    #[pallet::storage]
    #[pallet::getter(fn total_liquidity)]
    pub type TotalLiquidity<T: Config> = StorageMap<
        _, 
        Blake2_128Concat, 
        (T::AssetId, T::AssetId), 
        BalanceOf<T>, 
        ValueQuery
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Liquidity added to a pool. [who, asset_a, asset_b, amount_a, amount_b, liquidity]
        LiquidityAdded(T::AccountId, T::AssetId, T::AssetId, BalanceOf<T>, BalanceOf<T>, BalanceOf<T>),
        
        /// Liquidity removed from a pool. [who, asset_a, asset_b, amount_a, amount_b, liquidity]
        LiquidityRemoved(T::AccountId, T::AssetId, T::AssetId, BalanceOf<T>, BalanceOf<T>, BalanceOf<T>),
        
        /// Assets swapped. [who, asset_in, asset_out, amount_in, amount_out, fee_amount]
        Swap(T::AccountId, T::AssetId, T::AssetId, BalanceOf<T>, BalanceOf<T>, BalanceOf<T>),
        
        /// Fee collected from a swap. [asset, amount]
        FeeCollected(T::AssetId, BalanceOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Invalid asset pair (identical assets)
        InvalidAssetPair,
        
        /// Invalid amount (zero or overflow)
        InvalidAmount,
        
        /// Insufficient liquidity
        InsufficientLiquidity,
        
        /// Insufficient balance
        InsufficientBalance,
        
        /// Pool does not exist
        PoolDoesNotExist,
        
        /// Liquidity pool calculation error
        CalculationError,
        
        /// Slippage tolerance exceeded
        SlippageExceeded,
        
        /// Deadline exceeded
        DeadlineExceeded,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Add liquidity to a pool
        #[pallet::weight(10_000)]
        pub fn add_liquidity(
            origin: OriginFor<T>,
            asset_a: T::AssetId,
            asset_b: T::AssetId,
            amount_a: BalanceOf<T>,
            amount_b: BalanceOf<T>,
            min_liquidity: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Ensure the asset pair is valid
            ensure!(asset_a != asset_b, Error::<T>::InvalidAssetPair);

            // Ensure the amounts are non-zero
            ensure!(amount_a > Zero::zero() && amount_b > Zero::zero(), Error::<T>::InvalidAmount);

            // Fetch the current reserves
            let (reserve_a, reserve_b) = LiquidityPools::<T>::get((asset_a, asset_b));

            // Calculate the liquidity to be added
            let liquidity = if reserve_a.is_zero() && reserve_b.is_zero() {
                // Initial liquidity provision
                amount_a.min(amount_b)
            } else {
                // Calculate based on existing reserves
                let liquidity_a = amount_a
                    .checked_mul(&reserve_b)
                    .ok_or(Error::<T>::CalculationError)?
                    .checked_div(&reserve_a)
                    .ok_or(Error::<T>::CalculationError)?;
                let liquidity_b = amount_b
                    .checked_mul(&reserve_a)
                    .ok_or(Error::<T>::CalculationError)?
                    .checked_div(&reserve_b)
                    .ok_or(Error::<T>::CalculationError)?;
                liquidity_a.min(liquidity_b)
            };

            // Ensure the liquidity meets the minimum requirement
            ensure!(liquidity >= min_liquidity, Error::<T>::InsufficientLiquidity);

            // Reserve (transfer) tokens from the user
            T::Currency::transfer(
                &who, 
                &Self::account_id(), 
                amount_a, 
                ExistenceRequirement::KeepAlive
            )?;
            T::Currency::transfer(
                &who, 
                &Self::account_id(), 
                amount_b, 
                ExistenceRequirement::KeepAlive
            )?;

            // Update the liquidity pool reserves
            LiquidityPools::<T>::mutate((asset_a, asset_b), |reserves| {
                reserves.0 = reserves.0.saturating_add(amount_a);
                reserves.1 = reserves.1.saturating_add(amount_b);
            });

            // Mint liquidity tokens to the provider
            Self::mint_liquidity_tokens(&who, (asset_a, asset_b), liquidity)?;

            // Update total liquidity
            TotalLiquidity::<T>::mutate((asset_a, asset_b), |total| {
                *total = total.saturating_add(liquidity);
            });

            // Emit an event
            Self::deposit_event(Event::LiquidityAdded(who, asset_a, asset_b, amount_a, amount_b, liquidity));

            Ok(())
        }

        /// Remove liquidity from a pool
        #[pallet::weight(10_000)]
        pub fn remove_liquidity(
            origin: OriginFor<T>,
            asset_a: T::AssetId,
            asset_b: T::AssetId,
            liquidity: BalanceOf<T>,
            min_amount_a: BalanceOf<T>,
            min_amount_b: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Ensure the asset pair is valid
            ensure!(asset_a != asset_b, Error::<T>::InvalidAssetPair);

            // Ensure the liquidity amount is non-zero
            ensure!(liquidity > Zero::zero(), Error::<T>::InvalidAmount);

            // Fetch the current reserves
            let (reserve_a, reserve_b) = LiquidityPools::<T>::get((asset_a, asset_b));

            // Ensure the pool exists (has liquidity)
            ensure!(reserve_a > Zero::zero() && reserve_b > Zero::zero(), Error::<T>::PoolDoesNotExist);

            // Get the total liquidity of the pool
            let total_liquidity = TotalLiquidity::<T>::get((asset_a, asset_b));
            ensure!(total_liquidity > Zero::zero(), Error::<T>::PoolDoesNotExist);

            // Calculate the amounts to return
            let amount_a = liquidity
                .checked_mul(&reserve_a)
                .ok_or(Error::<T>::CalculationError)?
                .checked_div(&total_liquidity)
                .ok_or(Error::<T>::CalculationError)?;
            let amount_b = liquidity
                .checked_mul(&reserve_b)
                .ok_or(Error::<T>::CalculationError)?
                .checked_div(&total_liquidity)
                .ok_or(Error::<T>::CalculationError)?;

            // Ensure minimum amounts are satisfied
            ensure!(amount_a >= min_amount_a, Error::<T>::SlippageExceeded);
            ensure!(amount_b >= min_amount_b, Error::<T>::SlippageExceeded);

            // Burn the liquidity tokens
            Self::burn_liquidity_tokens(&who, (asset_a, asset_b), liquidity)?;

            // Update total liquidity
            TotalLiquidity::<T>::mutate((asset_a, asset_b), |total| {
                *total = total.saturating_sub(liquidity);
            });

            // Update the reserves
            LiquidityPools::<T>::mutate((asset_a, asset_b), |reserves| {
                reserves.0 = reserves.0.saturating_sub(amount_a);
                reserves.1 = reserves.1.saturating_sub(amount_b);
            });

            // Transfer the assets back to the user
            T::Currency::transfer(
                &Self::account_id(), 
                &who, 
                amount_a, 
                ExistenceRequirement::AllowDeath
            )?;
            T::Currency::transfer(
                &Self::account_id(), 
                &who, 
                amount_b, 
                ExistenceRequirement::AllowDeath
            )?;

            // Emit an event
            Self::deposit_event(Event::LiquidityRemoved(who, asset_a, asset_b, amount_a, amount_b, liquidity));

            Ok(())
        }

        /// Swap tokens using a liquidity pool with the Nourish Chain 0.369% fee
        #[pallet::weight(10_000)]
        pub fn swap(
            origin: OriginFor<T>,
            asset_in: T::AssetId,
            asset_out: T::AssetId,
            amount_in: BalanceOf<T>,
            min_amount_out: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Ensure the asset pair is valid
            ensure!(asset_in != asset_out, Error::<T>::InvalidAssetPair);

            // Ensure the amount is non-zero
            ensure!(amount_in > Zero::zero(), Error::<T>::InvalidAmount);

            // Determine the correct order of assets
            let (asset_a, asset_b, is_reversed) = if asset_in < asset_out {
                (asset_in, asset_out, false)
            } else {
                (asset_out, asset_in, true)
            };

            // Fetch the current reserves
            let (mut reserve_a, mut reserve_b) = LiquidityPools::<T>::get((asset_a, asset_b));

            // Ensure the pool exists
            ensure!(reserve_a > Zero::zero() && reserve_b > Zero::zero(), Error::<T>::PoolDoesNotExist);

            // Apply the reserves based on the swap direction
            let (reserve_in, reserve_out) = if is_reversed {
                (reserve_b, reserve_a)
            } else {
                (reserve_a, reserve_b)
            };

            // Calculate fee amount (0.369% of amount_in)
            let fee_amount = T::TradingFee::mul_floor(amount_in);
            let amount_in_after_fee = amount_in.saturating_sub(fee_amount);

            // Calculate amount out using the constant product formula: x * y = k
            // (reserve_in + amount_in_after_fee) * (reserve_out - amount_out) = reserve_in * reserve_out
            // So: amount_out = reserve_out - (reserve_in * reserve_out) / (reserve_in + amount_in_after_fee)
            let numerator = reserve_in
                .checked_mul(&reserve_out)
                .ok_or(Error::<T>::CalculationError)?;
            let denominator = reserve_in
                .checked_add(&amount_in_after_fee)
                .ok_or(Error::<T>::CalculationError)?;
            let new_reserve_out = numerator
                .checked_div(&denominator)
                .ok_or(Error::<T>::CalculationError)?;
            let amount_out = reserve_out.saturating_sub(new_reserve_out);

            // Ensure minimum amount out is satisfied
            ensure!(amount_out >= min_amount_out, Error::<T>::SlippageExceeded);

            // Reserve (transfer) tokens from the user
            T::Currency::transfer(
                &who, 
                &Self::account_id(), 
                amount_in, 
                ExistenceRequirement::KeepAlive
            )?;

            // Update the reserves
            if is_reversed {
                reserve_b = reserve_b.saturating_add(amount_in);
                reserve_a = reserve_a.saturating_sub(amount_out);
            } else {
                reserve_a = reserve_a.saturating_add(amount_in);
                reserve_b = reserve_b.saturating_sub(amount_out);
            }
            LiquidityPools::<T>::insert((asset_a, asset_b), (reserve_a, reserve_b));

            // Transfer the output assets to the user
            T::Currency::transfer(
                &Self::account_id(), 
                &who, 
                amount_out, 
                ExistenceRequirement::AllowDeath
            )?;

            // Emit events
            Self::deposit_event(Event::Swap(who, asset_in, asset_out, amount_in, amount_out, fee_amount));
            Self::deposit_event(Event::FeeCollected(asset_in, fee_amount));

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Get the account ID for the pallet
        fn account_id() -> T::AccountId {
            T::PalletId::get().into_account_truncating()
        }

        /// Mint liquidity tokens to a user
        fn mint_liquidity_tokens(
            who: &T::AccountId,
            pool: (T::AssetId, T::AssetId),
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            LiquidityTokens::<T>::mutate((pool.0, pool.1, who.clone()), |balance| {
                *balance = balance.saturating_add(amount);
            });
            Ok(())
        }

        /// Burn liquidity tokens from a user
        fn burn_liquidity_tokens(
            who: &T::AccountId,
            pool: (T::AssetId, T::AssetId),
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            LiquidityTokens::<T>::try_mutate((pool.0, pool.1, who.clone()), |balance| -> DispatchResult {
                ensure!(*balance >= amount, Error::<T>::InsufficientBalance);
                *balance = balance.saturating_sub(amount);
                Ok(())
            })
        }
    }
}
