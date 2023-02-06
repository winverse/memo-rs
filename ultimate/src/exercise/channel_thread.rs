use crossbeam::channel::{self, Receiver, Sender};
use std::{thread, time::Duration};

#[derive(Debug)]
enum Lunch {
    Soup,
    Salad,
    Sandwich,
    HotDog,
}

/// * `name` - 식당 직원들
/// * `orders` - 채널 수신 주문 목록
/// * `lunches` -
fn cafeteria_worker(name: &str, orders: Receiver<&str>, lunches: Sender<Lunch>) {
    for order in orders {
        println!("{} receives an order for {}", name, order);
        // order 참조를 사용하지 않고 order를 사용하게 되면 match 문이 끝나면서 order가 사라지게 됨
        let lunch = match &order {
            x if x.contains("soup") => Lunch::Soup,
            x if x.contains("sandwich") => Lunch::Sandwich,
            x if x.contains("salad") => Lunch::Salad,
            _ => Lunch::HotDog,
        };

        for _ in 0..order.len() {
            thread::sleep(Duration::from_secs_f32(0.1))
        }
        println!("{} sends a {:?}", name, lunch);
        if lunches.send(lunch).is_err() {
            break;
        }
    }
}

pub fn channel_thread() {
    // channel::unbound의 size를 정해서 크기를 결정해놔야지 문제가 안생김
    let (orders_tx, orders_rx) = channel::unbounded();
    let (lunches_tx, lunches_rx) = channel::unbounded();

    let orders_rx2 = orders_rx.clone();
    let lunches_tx2 = lunches_tx.clone();

    let zack_handle = thread::spawn(|| cafeteria_worker("zack", orders_rx, lunches_tx));
    let alice_handle = thread::spawn(|| cafeteria_worker("alice", orders_rx2, lunches_tx2));

    for order in vec![
        "polish dog",
        "caesar salad",
        "onion soup",
        "reuben sandwich",
    ] {
        println!("ORDER: {}", order);
        let _ = orders_tx.send(order);
    }
    drop(orders_tx);

    for lunch in lunches_rx {
        println!("Order Up! -> {:?}", lunch);
    }

    let _ = alice_handle.join();
    let _ = zack_handle.join();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_counting() {
        channel_thread()
        // assert_eq!(count_to_5() == 5, true);
    }
}
