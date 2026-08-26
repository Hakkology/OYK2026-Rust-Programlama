// Yapiyi gostermek icin yazilmis ornek: Customer/Order hic kullanilmiyor,
// bu yuzden 'never used' uyarilari normal.
#![allow(dead_code)]

pub mod product {
    pub use category::Category;

    #[derive(Debug, PartialEq)]
    pub(crate) struct Product {
        // the Product is only accessible in the current crate
        id: u64,
        name: String,
        price: f64,
        category: Category,
    }

    mod category {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum Category {
            Electronics,
            Clothing,
            Books,
        }
    }

    impl Product {
        pub fn new(id: u64, name: String, price: f64, category: Category) -> Self {
            Product { id, name, price, category }
        }

        fn calculate_tax(&self) -> f64 {
            self.price * 0.1
        }

        pub fn product_price(&self) -> f64 {
            self.price - self.calculate_tax()
        }
    }

    pub trait Intersect<T> {
        fn intersect(self, other: Vec<T>) -> Vec<T>;
    }

    impl<'a> Intersect<&'a Product> for Vec<&'a Product> {
        fn intersect(self, other: Vec<&'a Product>) -> Vec<&'a Product> {
            self.into_iter().filter(|p| other.contains(p)).collect()
        }
    }
}

pub mod customer {
    pub struct Customer {
        id: u64,
        name: String,
        email: String,
    }
}

pub mod order {
    use super::customer::Customer;
    use super::product::Product;
    struct Order {
        id: u64,
        product: Product,
        customer: Customer,
        quantity: u32,
    }

    impl Order {
        pub(self) fn calculate_discount(&self) -> f64 {
            if self.quantity > 5 { 0.1 } else { 0.0 }
        }

        pub fn total_bill(&self) -> f64 {
            let discount = self.calculate_discount();
            let total_before_discount = self.product.product_price() * self.quantity as f64;
            total_before_discount - (total_before_discount * discount)
        }
    }
}
