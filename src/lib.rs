pub mod commands;

#[derive(Debug)]
pub enum Size {
    Small,
    Medium,
    Large,
}

#[derive(Debug)]
pub enum Milk {
    None,
    Whole,
    Oat,
}

#[derive(Debug)]
pub enum Status {
    Pending,
    Preparing,
    Ready,
    Delivered,
}

#[derive(Debug)]
pub enum Discount {
    None,
    Percent(u32),
    Fixed(u32),
}

#[derive(Debug)]
pub struct Coffee {
    id: u16,
    name: String,
    size: Size,
    milk: Milk,
    status: Status,
    discount: Discount,
}

impl Coffee {
    pub fn price_calculate(&self) -> u32 {
        let mut price = 0;

        match self.size {
            Size::Small => price += 50,
            Size::Medium => price += 70,
            Size::Large => price += 90,
        }

        match self.milk {
            Milk::None => price += 0,
            Milk::Whole => price += 5,
            Milk::Oat => price += 10,
        }

        match self.discount {
            Discount::Percent(i) => {
                let i = i.min(100);
                price = price - (price * i) / 100;
            }
            Discount::Fixed(i) => price = price.saturating_sub(i),
            Discount::None => (),
        }

        price
    }

    pub fn advance_status(&mut self) {
        match self.status {
            Status::Pending => self.status = Status::Preparing,
            Status::Preparing => self.status = Status::Ready,
            Status::Ready => self.status = Status::Delivered,
            Status::Delivered => self.status = Status::Delivered,
        }
    }
    pub fn retreat_status(&mut self) {
        match self.status {
            Status::Pending => self.status = Status::Pending,
            Status::Preparing => self.status = Status::Pending,
            Status::Ready => self.status = Status::Preparing,
            Status::Delivered => self.status = Status::Ready,
        }
    }

    pub fn get_status(&self) -> String {
        format!("{:?}", self.status)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    // ---------- helper ----------
    fn coffee(size: Size, milk: Milk, discount: Discount) -> Coffee {
        Coffee {
            id: 1,
            name: "test".to_string(),
            size,
            milk,
            status: Status::Pending,
            discount,
        }
    }

    // ---------- price: size ----------
    #[test]
    fn price_small_no_extras() {
        let c = coffee(Size::Small, Milk::None, Discount::None);
        assert_eq!(c.price_calculate(), 50);
    }

    #[test]
    fn price_medium_no_extras() {
        let c = coffee(Size::Medium, Milk::None, Discount::None);
        assert_eq!(c.price_calculate(), 70);
    }

    #[test]
    fn price_large_no_extras() {
        let c = coffee(Size::Large, Milk::None, Discount::None);
        assert_eq!(c.price_calculate(), 90);
    }

    // ---------- price: milk ----------
    #[test]
    fn price_whole_milk_adds_5() {
        let c = coffee(Size::Small, Milk::Whole, Discount::None);
        assert_eq!(c.price_calculate(), 55);
    }

    #[test]
    fn price_oat_milk_adds_10() {
        let c = coffee(Size::Small, Milk::Oat, Discount::None);
        assert_eq!(c.price_calculate(), 60);
    }

    // ---------- price: discount ----------
    #[test]
    fn discount_percent_10() {
        let c = coffee(Size::Medium, Milk::None, Discount::Percent(10));
        // 70 - 7 = 63
        assert_eq!(c.price_calculate(), 63);
    }

    #[test]
    fn discount_percent_100_is_free() {
        let c = coffee(Size::Large, Milk::Oat, Discount::Percent(100));
        assert_eq!(c.price_calculate(), 0);
    }

    #[test]
    fn discount_percent_above_100_clamped() {
        let c = coffee(Size::Medium, Milk::None, Discount::Percent(200));
        // min(200, 100) = 100 → free
        assert_eq!(c.price_calculate(), 0);
    }

    #[test]
    fn discount_fixed_normal() {
        let c = coffee(Size::Medium, Milk::None, Discount::Fixed(20));
        // 70 - 20 = 50
        assert_eq!(c.price_calculate(), 50);
    }

    #[test]
    fn discount_fixed_larger_than_price_no_underflow() {
        let c = coffee(Size::Small, Milk::None, Discount::Fixed(500));
        // saturating_sub → 0
        assert_eq!(c.price_calculate(), 0);
    }

    #[test]
    fn discount_fixed_exactly_price() {
        let c = coffee(Size::Small, Milk::None, Discount::Fixed(50));
        assert_eq!(c.price_calculate(), 0);
    }

    // ---------- advance_status ----------
    #[test]
    fn advance_from_pending() {
        let mut c = coffee(Size::Small, Milk::None, Discount::None);
        c.advance_status();
        assert!(matches!(c.status, Status::Preparing));
    }

    #[test]
    fn advance_from_preparing() {
        let mut c = coffee(Size::Small, Milk::None, Discount::None);
        c.advance_status();
        c.advance_status();
        assert!(matches!(c.status, Status::Ready));
    }

    #[test]
    fn advance_from_ready() {
        let mut c = coffee(Size::Small, Milk::None, Discount::None);
        c.advance_status();
        c.advance_status();
        c.advance_status();
        assert!(matches!(c.status, Status::Delivered));
    }

    #[test]
    fn advance_from_delivered_stays_delivered() {
        let mut c = coffee(Size::Small, Milk::None, Discount::None);
        for _ in 0..5 {
            c.advance_status();
        }
        assert!(matches!(c.status, Status::Delivered));
    }

    // ---------- retreat_status ----------
    #[test]
    fn retreat_from_delivered() {
        let mut c = coffee(Size::Small, Milk::None, Discount::None);
        c.status = Status::Delivered;
        c.retreat_status();
        assert!(matches!(c.status, Status::Ready));
    }

    #[test]
    fn retreat_from_ready() {
        let mut c = coffee(Size::Small, Milk::None, Discount::None);
        c.status = Status::Ready;
        c.retreat_status();
        assert!(matches!(c.status, Status::Preparing));
    }

    #[test]
    fn retreat_from_preparing() {
        let mut c = coffee(Size::Small, Milk::None, Discount::None);
        c.status = Status::Preparing;
        c.retreat_status();
        assert!(matches!(c.status, Status::Pending));
    }

    #[test]
    fn retreat_from_pending_stays_pending() {
        let mut c = coffee(Size::Small, Milk::None, Discount::None);
        c.retreat_status();
        assert!(matches!(c.status, Status::Pending));
    }

    // ---------- full cycle ----------
    #[test]
    fn advance_then_retreat_returns_to_pending() {
        let mut c = coffee(Size::Small, Milk::None, Discount::None);
        c.advance_status();
        c.advance_status();
        c.advance_status(); // Delivered
        c.retreat_status();
        c.retreat_status();
        c.retreat_status(); // Pending
        assert!(matches!(c.status, Status::Pending));
    }

    // ---------- get_status ----------
    #[test]
    fn get_status_returns_debug_string() {
        let c = coffee(Size::Small, Milk::None, Discount::None);
        assert_eq!(c.get_status(), "Pending");
    }
}