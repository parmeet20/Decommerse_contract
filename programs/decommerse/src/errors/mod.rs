use anchor_lang::prelude::*;
#[error_code]
pub enum EcommerceError {
    #[msg("Username exceeds maximum length")]
    UsernameTooLong,
    #[msg("ArithmeticUnderflow")]
    ArithmeticUnderflow,
    #[msg("InvalidSeller")]
    InvalidSeller,
    #[msg("product not found")]
    ProductNotFound,
    #[msg("Program already intialized")]
    ProgramAlreadyInitialzed,
    #[msg("Product ID exceeds maximum length")]
    ProductIdTooLong,
    #[msg("Product name exceeds maximum length")]
    ProductNameTooLong,
    #[msg("Profile already intialized")]
    ProfileAlreadyInitialized,
    #[msg("Price must be greater than zero")]
    InvalidPrice,
    #[msg("Stock must be greater than zero")]
    InvalidStock,
    #[msg("Invalid quantity")]
    InvalidQuantity,
    #[msg("Insufficient stock for purchase")]
    InsufficientStock,
    #[msg("Price calculation overflow")]
    PriceOverflow,
    #[msg("Stock calculation underflow")]
    StockUnderflow,
    #[msg("Unauthorized access")]
    Unauthorized,
}