use itertools::Itertools;

/// Returns the fuel cost for the crabs to move regarding the current boat position
fn cost(boat_pos: usize, crab_pos: &Vec<i32>) -> i32{
    return crab_pos.iter().map(|i| i32::abs(boat_pos as i32 - i))
        .map(|n| (n * n + n) / 2).sum();
}

fn part2() {
    let content = include_str!("input.txt");
    let it = content.split(',')
        .map(str::parse::<i32>)
        .map(|i| i.unwrap())
        .sorted().collect_vec();
    let count = it.len();

    // There is one (discrete) minimum. Find it numerically
    let mut upper: usize = *it.iter().max().unwrap() as usize;
    let mut lower: usize = 0;

    loop {
        let mid = (upper + lower) / 2;
        let grad_upper = cost(upper + 1, &it) - cost(upper, &it);
        let grad_lower = cost(lower + 1, &it) - cost(lower, &it);
        let grad_mid = cost(mid + 1, &it) - cost(mid, &it);
        println!("Grad Low: {}, Grad mid: {}, Grad high: {}", grad_lower, grad_mid, grad_upper);

        if lower >= mid || upper <= mid {
            println!("Solution found!");
            println!("Lower: {}, Mid: {}, Upper: {}", lower, mid, upper);
            let cost1 = cost(mid - 1, &it);
            let cost2 = cost(mid, &it);
            let cost3 = cost(mid + 1, &it);
            println!("Minimum cost: {}", cost1.min(cost2).min(cost3));
            break;
        }

        if grad_mid.signum() * grad_lower.signum() < 0 {
            // [lower, mid] is new interval
            println!("[Low, Mid]");
            upper = mid;
        } else if grad_mid.signum() * grad_upper.signum() < 0{
            // [mid, upper] is new interval
            println!("[Mid, High]");
            lower = mid;
        } else {
            break;
        }
    }

}

fn part1() {
    let content = include_str!("input.txt");
    let it = content.split(',')
        .map(str::parse::<i32>)
        .map(|i| i.unwrap())
        .sorted().collect_vec();
    let count = it.len();
    // Possibly two means. But not in this case, so ignore it :)
    // If this were an issue, select the mean with the most numbers
    let mean = it[count/2];
    let fuel: i32 = it.iter()
        .map(|i| i32::abs(i - mean))
        .sum();

    println!("Count: {}", count);
    println!("mean = {}", mean);
    println!("Fuel = {}", fuel);

}
fn main() {
    part1();
    part2();
}
