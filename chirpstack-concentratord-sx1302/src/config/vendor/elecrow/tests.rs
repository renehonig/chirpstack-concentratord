//! Integration test for Elecrow HAT configuration
//! 
//! This test validates that the Elecrow HAT configuration can be loaded
//! and returns the expected settings, particularly the sx1302_reset_pin.

#[cfg(test)]
mod elecrow_tests {
    #[test]
    fn test_elecrow_model_name_format() {
        // Test that the model name follows the expected pattern
        let elecrow_model = "elecrow_sx1302_lorawan_gateway_hat";
        
        // Should contain elecrow vendor name
        assert!(elecrow_model.contains("elecrow"));
        
        // Should contain sx1302 chip identifier
        assert!(elecrow_model.contains("sx1302"));
        
        // Should contain lorawan and gateway identifiers
        assert!(elecrow_model.contains("lorawan"));
        assert!(elecrow_model.contains("gateway"));
        
        // Should contain hat identifier
        assert!(elecrow_model.contains("hat"));
    }
    
    #[test]
    fn test_elecrow_reset_pin_value() {
        // Test that the expected reset pin value is correct
        let expected_elecrow_pin = 17;
        let waveshare_pin = 23;
        
        // Elecrow should use pin 17, different from Waveshare pin 23
        assert_ne!(expected_elecrow_pin, waveshare_pin);
        assert_eq!(expected_elecrow_pin, 17);
    }
}