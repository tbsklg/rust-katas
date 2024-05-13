fn main() {
    choose_best_sum(163, 3, &vec![50, 55, 56, 57, 58]);
}

fn choose_best_sum(_t: i32, _k: i32, _ls: &Vec<i32>) -> i32 {
    return routes(_k, _ls)
        .iter()
        .map(|route| route.iter().sum())
        .filter(|&sum| sum <= _t)
        .max()
        .unwrap_or(-1);
}

fn routes(max_towns: i32, distances: &Vec<i32>) -> Vec<Vec<i32>> {
    match (max_towns, distances.as_slice()) {
        (0, []) => vec![vec![]],
        (_, []) => vec![],
        (n, xs) => {
            let head = xs[0];
            let tail = &xs[1..].to_vec();

            routes(n - 1, tail)
                .iter()
                .flat_map(|route| {
                    let mut new_route = vec![head];
                    new_route.extend(route.iter().cloned());
                    vec![new_route]
                })
                .chain(routes(n, tail).iter().cloned())
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::choose_best_sum;
    use crate::routes;

    fn testing(t: i32, k: i32, ls: &Vec<i32>, exp: i32) {
        assert_eq!(choose_best_sum(t, k, ls), exp)
    }

    #[test]
    fn basics_choose_best_sum() {
        let ts = &vec![50, 55, 56, 57, 58];
        testing(163, 3, ts, 163);
        let ts = &vec![50];
        testing(163, 3, ts, -1);
        let ts = &vec![91, 74, 73, 85, 73, 81, 87];
        testing(230, 3, ts, 228);
        testing(331, 2, ts, 178);
    }

    #[test]
    fn it_should_return_empty_routes_for_no_towns_and_empty_distances() {
        assert_eq!(routes(0, &vec![]), vec![vec![]] as Vec<Vec<i32>>);
    }

    #[test]
    fn it_should_return_empty_routes_for_no_towns_and_distances() {
        assert_eq!(routes(0, &vec![1, 2, 3]), vec![vec![]] as Vec<Vec<i32>>);
    }

    #[test]
    fn it_should_return_empty_routes_for_towns_and_no_distances() {
        assert_eq!(routes(3, &vec![]), vec![] as Vec<Vec<i32>>);
    }

    #[test]
    fn it_should_return_routes_for_towns_and_distances() {
        assert_eq!(
            routes(3, &vec![1, 2, 3]),
            vec![vec![1, 2, 3]] as Vec<Vec<i32>>
        );

        assert_eq!(
            routes(3, &vec![50, 55, 57, 58, 60]),
            vec![
                [50, 55, 57],
                [50, 55, 58],
                [50, 55, 60],
                [50, 57, 58],
                [50, 57, 60],
                [50, 58, 60],
                [55, 57, 58],
                [55, 57, 60],
                [55, 58, 60],
                [57, 58, 60]
            ],
        )
    }
}
