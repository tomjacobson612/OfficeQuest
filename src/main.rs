mod models;
use models::card;

fn main() {
    let test_card = card::friday_meeting();
    test_card.print();
}
