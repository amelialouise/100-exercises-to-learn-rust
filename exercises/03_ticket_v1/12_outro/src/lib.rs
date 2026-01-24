// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is numeric and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

// Technically we get to traits next, but just makes sense to use it here. 
pub trait Zero {
    const ZERO: Self;
}

impl Zero for u32 { const ZERO: Self = 0; }
impl Zero for f32 { const ZERO: Self = 0.0; }

pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: f32    
}

impl Order {

    pub fn is_zero<T>(value: T) -> bool 
        where T: PartialEq + Zero,
        {value == T::ZERO}

    pub fn new(product_name: String, quantity: u32, unit_price: f32) -> Order {
        Self::check_product_name(&product_name);
        Self::check_quantity(quantity);
        Self::check_unit_price(unit_price);

        Order {
            product_name,
            quantity,
            unit_price
        }
    }
    
    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &f32 {
        &self.unit_price
    }

    fn check_product_name(product_name: &String) {
        if product_name.is_empty(){
            panic!("Product name cannot be empty");
        }
        if product_name.len() > 300 {
            panic!("Product name cannot be longer than 300 bytes")
        }
    }

    fn check_quantity(quantity: u32) {
        if Self::is_zero(quantity) {
            panic!("Quantity must be strictly greater than zero")
        }
    }

    fn check_unit_price(unit_price: f32) {
        if Self::is_zero(unit_price) {
            panic!("Unit price must be strictly greater than zero")
        }
    }

    pub fn set_product_name(&mut self, new_product_name: String) {
        Self::check_product_name(&new_product_name);
        self.product_name = new_product_name;
    }

    pub fn set_quantity(&mut self, new_quantity: u32) {
        Self::check_quantity(self.quantity);
        self.quantity = new_quantity;
    }

    pub fn set_unit_price(&mut self, new_unit_price: f32) {
        Self::check_unit_price(self.unit_price);
        self.unit_price = new_unit_price;
    }

    pub fn total(&self) -> f32 {
        &self.unit_price * self.quantity as f32
    }
}