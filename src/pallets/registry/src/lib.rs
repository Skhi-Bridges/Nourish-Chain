#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod spirulina_registry {
    use ink_prelude::string::String;
    use ink_prelude::vec::Vec;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
    };
    use scale::{Decode, Encode};

    /// Represents a registered spirulina cultivation facility
    #[derive(Debug, Encode, Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct CultivationFacility {
        /// Unique ID for the facility
        id: String,
        /// Public name of the facility
        name: String,
        /// Geographic coordinates
        location: (i32, i32),
        /// Cultivation capacity in square meters
        capacity: u32,
        /// ISO certification details
        certifications: Vec<Certification>,
        /// Cultivation methods used
        methods: Vec<CultivationMethod>,
        /// Status of the facility
        status: FacilityStatus,
        /// Owner account
        owner: AccountId,
        /// Last audit timestamp
        last_audit: Timestamp,
    }

    /// Certification information
    #[derive(Debug, Encode, Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Certification {
        /// Type of certification
        cert_type: CertificationType,
        /// Unique certification ID
        cert_id: String,
        /// Who issued the certification
        issuer: String,
        /// When the certification expires
        valid_until: Timestamp,
    }

    /// Types of certifications
    #[derive(Debug, Encode, Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum CertificationType {
        /// ISO-9001 Quality Management
        ISO9001,
        /// ISO-14001 Environmental Management
        ISO14001,
        /// ISO-22000 Food Safety Management
        ISO22000,
        /// Organic certification
        Organic,
        /// Fair trade certification
        FairTrade,
        /// GMP (Good Manufacturing Practice)
        GMP,
        /// HACCP (Hazard Analysis Critical Control Point)
        HACCP,
        /// Non-GMO certification
        NonGMO,
        /// USP (United States Pharmacopeia) certification
        USP,
        /// Kosher certification
        Kosher,
        /// Halal certification
        Halal,
        /// GRAS (Generally Recognized As Safe) certification
        GRAS,
        /// Custom certification type
        Custom(String),
    }

    /// Cultivation methods
    #[derive(Debug, Encode, Decode, Clone, SpreadLayout, PackedLayout, PartialEq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum CultivationMethod {
        /// Open pond cultivation
        OpenPond,
        /// Closed bioreactor
        ClosedBioreactor,
        /// Hybrid system
        Hybrid,
        /// Photobioreactor
        Photobioreactor,
        /// Custom method
        Custom(String),
    }

    /// Facility status
    #[derive(Debug, Encode, Decode, Clone, SpreadLayout, PackedLayout, PartialEq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum FacilityStatus {
        /// Application submitted, pending verification
        Pending,
        /// Facility verified and active
        Active,
        /// Facility temporarily suspended
        Suspended,
        /// Facility permanently revoked
        Revoked,
        /// Facility undergoing audit
        UnderAudit,
    }

    /// Represents an authorized telemetry device
    #[derive(Debug, Encode, Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct TelemetryDevice {
        /// Unique device ID
        id: String,
        /// Facility this device is associated with
        facility_id: String,
        /// Public key for the device (for verifying signatures)
        public_key: Vec<u8>,
        /// Current firmware version
        firmware_version: String,
        /// Current status
        status: DeviceStatus,
        /// Last activity timestamp
        last_active: Timestamp,
    }

    /// Status of a telemetry device
    #[derive(Debug, Encode, Decode, Clone, SpreadLayout, PackedLayout, PartialEq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum DeviceStatus {
        /// Device is registered but not yet authorized
        Registered,
        /// Device is authorized and active
        Authorized,
        /// Device is suspended (e.g., suspicious activity)
        Suspended,
        /// Device has been decommissioned
        Decommissioned,
    }

    /// Cultivation parameters for a facility
    #[derive(Debug, Encode, Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct CultivationParameters {
        /// Water pH level
        ph_level: u32, // Stored as pH * 100 (e.g., 7.50 = 750)
        /// Temperature in celsius * 100
        temperature: i32,
        /// Light intensity (lux)
        light_intensity: u32,
        /// CO2 concentration (ppm)
        co2_concentration: u32,
        /// Nutrient concentration (mg/L * 100)
        nutrient_concentration: u32,
        /// Water quality metrics
        water_quality: u32, // 0-10000 scale
    }

    /// Simple timestamp type (Unix timestamp)
    pub type Timestamp = u64;

    #[ink(storage)]
    pub struct SpirulinaRegistry {
        /// Contract owner
        owner: AccountId,
        /// Map of facility ID to facility
        facilities: StorageHashMap<String, CultivationFacility>,
        /// Map of device ID to device
        devices: StorageHashMap<String, TelemetryDevice>,
        /// Map of facility ID to cultivation parameters
        parameters: StorageHashMap<String, CultivationParameters>,
        /// Map of user to array of facility IDs they own
        owned_facilities: StorageHashMap<AccountId, Vec<String>>,
        /// Default parameters for new facilities
        default_parameters: CultivationParameters,
        /// Authorized auditors
        auditors: Vec<AccountId>,
    }

    /// Errors that can occur in the registry
    #[derive(Debug, Encode, Decode, PartialEq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Only the contract owner can perform this action
        NotOwner,
        /// Only the facility owner can perform this action
        NotFacilityOwner,
        /// Only authorized auditors can perform this action
        NotAuditor,
        /// Facility with this ID already exists
        FacilityAlreadyExists,
        /// Facility with this ID does not exist
        FacilityNotFound,
        /// Device with this ID already exists
        DeviceAlreadyExists,
        /// Device with this ID does not exist
        DeviceNotFound,
        /// Device is not associated with the specified facility
        DeviceNotAssociated,
        /// Parameters are invalid or out of acceptable range
        InvalidParameters,
        /// Account is already an auditor
        AlreadyAuditor,
        /// Account is not an auditor
        NotAnAuditor,
    }

    /// Events emitted by the contract
    #[ink(event)]
    pub struct FacilityRegistered {
        #[ink(topic)]
        facility_id: String,
        owner: AccountId,
    }

    #[ink(event)]
    pub struct FacilityStatusChanged {
        #[ink(topic)]
        facility_id: String,
        new_status: FacilityStatus,
    }

    #[ink(event)]
    pub struct DeviceAuthorized {
        #[ink(topic)]
        device_id: String,
        #[ink(topic)]
        facility_id: String,
    }

    #[ink(event)]
    pub struct DeviceStatusChanged {
        #[ink(topic)]
        device_id: String,
        new_status: DeviceStatus,
    }

    #[ink(event)]
    pub struct ParametersUpdated {
        #[ink(topic)]
        facility_id: String,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl SpirulinaRegistry {
        /// Creates a new registry with the caller as owner
        #[ink(constructor)]
        pub fn new() -> Self {
            // Default parameters for new facilities
            let default_parameters = CultivationParameters {
                ph_level: 750, // pH 7.5
                temperature: 3000, // 30.00 C
                light_intensity: 20000, // 20,000 lux
                co2_concentration: 400, // 400 ppm
                nutrient_concentration: 1500, // 15.00 mg/L
                water_quality: 8000, // 80% quality
            };

            Self {
                owner: Self::env().caller(),
                facilities: StorageHashMap::new(),
                devices: StorageHashMap::new(),
                parameters: StorageHashMap::new(),
                owned_facilities: StorageHashMap::new(),
                default_parameters,
                auditors: Vec::new(),
            }
        }

        /// Registers a new cultivation facility
        #[ink(message)]
        pub fn register_facility(
            &mut self,
            id: String,
            name: String,
            location: (i32, i32),
            capacity: u32,
            methods: Vec<CultivationMethod>,
        ) -> Result<()> {
            // Ensure facility ID doesn't exist
            if self.facilities.contains_key(&id) {
                return Err(Error::FacilityAlreadyExists);
            }

            let caller = self.env().caller();
            let now = self.env().block_timestamp();

            // Create the new facility
            let facility = CultivationFacility {
                id: id.clone(),
                name,
                location,
                capacity,
                certifications: Vec::new(),
                methods,
                status: FacilityStatus::Pending,
                owner: caller,
                last_audit: now,
            };

            // Store the facility
            self.facilities.insert(id.clone(), facility);

            // Update owned facilities
            let mut owned = self.owned_facilities.get(&caller).unwrap_or(&Vec::new()).clone();
            owned.push(id.clone());
            self.owned_facilities.insert(caller, owned);

            // Add default parameters
            self.parameters.insert(id.clone(), self.default_parameters.clone());

            // Emit event
            self.env().emit_event(FacilityRegistered {
                facility_id: id,
                owner: caller,
            });

            Ok(())
        }

        /// Updates the status of a facility
        #[ink(message)]
        pub fn update_facility_status(
            &mut self,
            facility_id: String,
            new_status: FacilityStatus,
        ) -> Result<()> {
            // Only owner or auditor can update status
            let caller = self.env().caller();
            if caller != self.owner && !self.auditors.contains(&caller) {
                return Err(Error::NotAuditor);
            }

            // Get the facility
            let facility = self.facilities.get_mut(&facility_id).ok_or(Error::FacilityNotFound)?;

            // Update the status
            facility.status = new_status.clone();

            // Emit event
            self.env().emit_event(FacilityStatusChanged {
                facility_id,
                new_status,
            });

            Ok(())
        }

        /// Registers a new telemetry device for a facility
        #[ink(message)]
        pub fn register_device(
            &mut self,
            device_id: String,
            facility_id: String,
            public_key: Vec<u8>,
            firmware_version: String,
        ) -> Result<()> {
            // Ensure device ID doesn't exist
            if self.devices.contains_key(&device_id) {
                return Err(Error::DeviceAlreadyExists);
            }

            // Ensure facility exists
            let facility = self.facilities.get(&facility_id).ok_or(Error::FacilityNotFound)?;

            // Ensure caller is facility owner or contract owner
            let caller = self.env().caller();
            if caller != facility.owner && caller != self.owner {
                return Err(Error::NotFacilityOwner);
            }

            let now = self.env().block_timestamp();

            // Create device
            let device = TelemetryDevice {
                id: device_id.clone(),
                facility_id: facility_id.clone(),
                public_key,
                firmware_version,
                status: DeviceStatus::Registered,
                last_active: now,
            };

            // Store the device
            self.devices.insert(device_id.clone(), device);

            // Automatically authorize the device if added by facility owner
            if caller == facility.owner {
                self.env().emit_event(DeviceAuthorized {
                    device_id: device_id.clone(),
                    facility_id: facility_id.clone(),
                });

                // Update the status to Authorized
                let device = self.devices.get_mut(&device_id).unwrap();
                device.status = DeviceStatus::Authorized;

                self.env().emit_event(DeviceStatusChanged {
                    device_id,
                    new_status: DeviceStatus::Authorized,
                });
            }

            Ok(())
        }

        /// Updates device status
        #[ink(message)]
        pub fn update_device_status(
            &mut self,
            device_id: String,
            new_status: DeviceStatus,
        ) -> Result<()> {
            // Get the device
            let device = self.devices.get(&device_id).ok_or(Error::DeviceNotFound)?;

            // Get the facility
            let facility = self.facilities.get(&device.facility_id).ok_or(Error::FacilityNotFound)?;

            // Ensure caller is facility owner, device owner, or contract owner
            let caller = self.env().caller();
            if caller != facility.owner && caller != self.owner {
                return Err(Error::NotFacilityOwner);
            }

            // Update the status
            let device = self.devices.get_mut(&device_id).unwrap();
            device.status = new_status.clone();

            // Emit event
            self.env().emit_event(DeviceStatusChanged {
                device_id,
                new_status,
            });

            Ok(())
        }

        /// Updates cultivation parameters for a facility
        #[ink(message)]
        pub fn update_parameters(
            &mut self,
            facility_id: String,
            parameters: CultivationParameters,
        ) -> Result<()> {
            // Ensure parameters are valid
            if !self.is_valid_parameters(&parameters) {
                return Err(Error::InvalidParameters);
            }

            // Ensure facility exists
            let facility = self.facilities.get(&facility_id).ok_or(Error::FacilityNotFound)?;

            // Ensure caller is facility owner, auditor, or contract owner
            let caller = self.env().caller();
            if caller != facility.owner && caller != self.owner && !self.auditors.contains(&caller) {
                return Err(Error::NotFacilityOwner);
            }

            // Update parameters
            self.parameters.insert(facility_id.clone(), parameters);

            // Emit event
            self.env().emit_event(ParametersUpdated {
                facility_id,
            });

            Ok(())
        }

        /// Adds a certification to a facility
        #[ink(message)]
        pub fn add_certification(
            &mut self,
            facility_id: String,
            cert_type: CertificationType,
            cert_id: String,
            issuer: String,
            valid_until: Timestamp,
        ) -> Result<()> {
            // Only owner, facility owner, or auditor can add certification
            let caller = self.env().caller();
            
            // Get the facility
            let facility = self.facilities.get_mut(&facility_id).ok_or(Error::FacilityNotFound)?;
            
            if caller != self.owner && caller != facility.owner && !self.auditors.contains(&caller) {
                return Err(Error::NotAuditor);
            }

            // Create certification
            let certification = Certification {
                cert_type,
                cert_id,
                issuer,
                valid_until,
            };

            // Add to facility
            facility.certifications.push(certification);

            Ok(())
        }

        /// Performs an audit on a facility
        #[ink(message)]
        pub fn perform_audit(
            &mut self,
            facility_id: String,
        ) -> Result<()> {
            // Only auditors can perform audits
            let caller = self.env().caller();
            if !self.auditors.contains(&caller) {
                return Err(Error::NotAuditor);
            }

            // Get the facility
            let facility = self.facilities.get_mut(&facility_id).ok_or(Error::FacilityNotFound)?;

            // Update status to under audit
            let old_status = facility.status.clone();
            facility.status = FacilityStatus::UnderAudit;

            // Update last audit timestamp
            facility.last_audit = self.env().block_timestamp();

            // Emit event
            self.env().emit_event(FacilityStatusChanged {
                facility_id,
                new_status: FacilityStatus::UnderAudit,
            });

            Ok(())
        }

        /// Adds an auditor
        #[ink(message)]
        pub fn add_auditor(
            &mut self,
            auditor: AccountId,
        ) -> Result<()> {
            // Only owner can add auditors
            if self.env().caller() != self.owner {
                return Err(Error::NotOwner);
            }

            // Ensure not already an auditor
            if self.auditors.contains(&auditor) {
                return Err(Error::AlreadyAuditor);
            }

            // Add the auditor
            self.auditors.push(auditor);

            Ok(())
        }

        /// Removes an auditor
        #[ink(message)]
        pub fn remove_auditor(
            &mut self,
            auditor: AccountId,
        ) -> Result<()> {
            // Only owner can remove auditors
            if self.env().caller() != self.owner {
                return Err(Error::NotOwner);
            }

            // Find and remove the auditor
            let pos = self.auditors.iter().position(|a| a == &auditor).ok_or(Error::NotAnAuditor)?;
            self.auditors.swap_remove(pos);

            Ok(())
        }

        /// Updates the default parameters for new facilities
        #[ink(message)]
        pub fn update_default_parameters(
            &mut self,
            parameters: CultivationParameters,
        ) -> Result<()> {
            // Only owner can update default parameters
            if self.env().caller() != self.owner {
                return Err(Error::NotOwner);
            }

            // Ensure parameters are valid
            if !self.is_valid_parameters(&parameters) {
                return Err(Error::InvalidParameters);
            }

            // Update default parameters
            self.default_parameters = parameters;

            Ok(())
        }

        /// Checks if an account is an authorized auditor
        #[ink(message)]
        pub fn is_auditor(&self, account: AccountId) -> bool {
            self.auditors.contains(&account)
        }

        /// Gets a facility by ID
        #[ink(message)]
        pub fn get_facility(&self, facility_id: String) -> Option<CultivationFacility> {
            self.facilities.get(&facility_id).cloned()
        }

        /// Gets a device by ID
        #[ink(message)]
        pub fn get_device(&self, device_id: String) -> Option<TelemetryDevice> {
            self.devices.get(&device_id).cloned()
        }

        /// Gets cultivation parameters for a facility
        #[ink(message)]
        pub fn get_parameters(&self, facility_id: String) -> Option<CultivationParameters> {
            self.parameters.get(&facility_id).cloned()
        }

        /// Gets the default parameters
        #[ink(message)]
        pub fn get_default_parameters(&self) -> CultivationParameters {
            self.default_parameters.clone()
        }

        /// Gets facilities owned by an account
        #[ink(message)]
        pub fn get_facilities_by_owner(&self, owner: AccountId) -> Vec<String> {
            self.owned_facilities.get(&owner).cloned().unwrap_or_default()
        }

        /// Gets the total number of registered facilities
        #[ink(message)]
        pub fn get_facilities_count(&self) -> u32 {
            self.facilities.len() as u32
        }

        /// Gets the total number of authorized devices
        #[ink(message)]
        pub fn get_devices_count(&self) -> u32 {
            self.devices.len() as u32
        }

        /// Updates the activity timestamp for a device
        #[ink(message)]
        pub fn update_device_activity(&mut self, device_id: String) -> Result<()> {
            // Get the device
            let device = self.devices.get_mut(&device_id).ok_or(Error::DeviceNotFound)?;

            // Only update if device is authorized
            if device.status != DeviceStatus::Authorized {
                return Ok(());
            }

            // Update last active timestamp
            device.last_active = self.env().block_timestamp();

            Ok(())
        }

        /// Validates if a device is authorized for a specific facility
        #[ink(message)]
        pub fn is_device_authorized(&self, device_id: String, facility_id: String) -> bool {
            if let Some(device) = self.devices.get(&device_id) {
                return device.facility_id == facility_id && device.status == DeviceStatus::Authorized;
            }
            
            false
        }

        /// Validates that parameters are within reasonable bounds
        fn is_valid_parameters(&self, parameters: &CultivationParameters) -> bool {
            // pH should be between 6.0 and 9.0 (600-900)
            if parameters.ph_level < 600 || parameters.ph_level > 900 {
                return false;
            }

            // Temperature should be between 20C and 40C (2000-4000)
            if parameters.temperature < 2000 || parameters.temperature > 4000 {
                return false;
            }

            // Light intensity should be between 5,000 and 50,000 lux
            if parameters.light_intensity < 5000 || parameters.light_intensity > 50000 {
                return false;
            }

            // CO2 should be between 300 and 1500 ppm
            if parameters.co2_concentration < 300 || parameters.co2_concentration > 1500 {
                return false;
            }

            // Nutrient concentration should be between 5.00 and 30.00 mg/L (500-3000)
            if parameters.nutrient_concentration < 500 || parameters.nutrient_concentration > 3000 {
                return false;
            }

            // Water quality should be between 50% and 100% (5000-10000)
            if parameters.water_quality < 5000 || parameters.water_quality > 10000 {
                return false;
            }

            true
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn registry_works() {
            let mut registry = SpirulinaRegistry::new();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // Register a facility
            registry.register_facility(
                String::from("fac_001"),
                String::from("Test Facility"),
                (123, 456),
                10000,
                vec![CultivationMethod::OpenPond],
            ).unwrap();

            // Check facility count
            assert_eq!(registry.get_facilities_count(), 1);

            // Add auditor
            registry.add_auditor(accounts.bob).unwrap();
            assert!(registry.is_auditor(accounts.bob));

            // Get facility
            let facility = registry.get_facility(String::from("fac_001")).unwrap();
            assert_eq!(facility.name, String::from("Test Facility"));
            assert_eq!(facility.status, FacilityStatus::Pending);
        }
    }
}
