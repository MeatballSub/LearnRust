#![feature(test)]

use std::collections::VecDeque;
use std::default;

use crate::list::LinkedList;
use rand::prelude::*;

mod list;

fn init() -> Vec<usize>
{
    let mut rng = rand::thread_rng();
    //(1..10).map(|_| rng.gen()).collect()
    (1..10000).map(|_| rng.gen()).collect()
}

fn init_sorted() -> Vec<usize> { (1..10000).collect() }

fn init_reverse_sorted() -> Vec<usize> { (1..10000).map(|v| 100000 - v).collect() }

fn list_insertion_sort(random_values: &Vec<usize>)
{
    let mut list = LinkedList::<usize>::new();
    for value in random_values
    {
        list.insert_sorted(*value);
    }
}

fn vec_insertion_sort(random_values: &Vec<usize>)
{
    let mut my_vec: Vec<usize> = vec![];
    for value in random_values
    {
        if value >= my_vec.last().unwrap_or(&0)
        {
            my_vec.push(*value);
        }
        else
        {
            let pos = my_vec.iter()
                            .position(|elem| elem >= value)
                            .unwrap_or(my_vec.len());
            my_vec.insert(pos, *value);
        }
    }
}

fn vecdeque_insertion_sort(random_values: &Vec<usize>)
{
    let mut my_vec: VecDeque<usize> = VecDeque::<usize>::new();
    for value in random_values
    {
        if value >= my_vec.back().unwrap_or(&0)
        {
            my_vec.push_back(*value);
        }
        else if value <= my_vec.front().unwrap_or(&usize::MAX)
        {
            my_vec.push_front(*value);
        }
        else
        {
            let pos = my_vec.iter()
                            .position(|elem| elem >= value)
                            .unwrap_or(my_vec.len());
            my_vec.insert(pos, *value);
        }
    }
}

fn main()
{
    let random_values = init();
    vec_insertion_sort(&random_values);
}

extern crate test;

#[cfg(test)]
mod tests
{
    use super::*;
    use test::Bencher;

    #[bench]
    fn test_list(bencher: &mut Bencher)
    {
        let random_values = init();
        bencher.iter(|| list_insertion_sort(&random_values));
    }

    #[bench]
    fn test_vec(bencher: &mut Bencher)
    {
        let random_values = init();
        bencher.iter(|| vec_insertion_sort(&random_values));
    }

    #[bench]
    fn test_vecdeque(bencher: &mut Bencher)
    {
        let random_values = init();
        bencher.iter(|| vecdeque_insertion_sort(&random_values));
    }

    #[bench]
    fn test_list_sorted(bencher: &mut Bencher)
    {
        let random_values = init_sorted();
        bencher.iter(|| list_insertion_sort(&random_values));
    }

    #[bench]
    fn test_vec_sorted(bencher: &mut Bencher)
    {
        let random_values = init_sorted();
        bencher.iter(|| vec_insertion_sort(&random_values));
    }

    #[bench]
    fn test_vecdeque_sorted(bencher: &mut Bencher)
    {
        let random_values = init_sorted();
        bencher.iter(|| vecdeque_insertion_sort(&random_values));
    }

    #[bench]
    fn test_list_reverse_sorted(bencher: &mut Bencher)
    {
        let random_values = init_reverse_sorted();
        bencher.iter(|| list_insertion_sort(&random_values));
    }

    #[bench]
    fn test_vec_reverse_sorted(bencher: &mut Bencher)
    {
        let random_values = init_reverse_sorted();
        bencher.iter(|| vec_insertion_sort(&random_values));
    }

    #[bench]
    fn test_vecdeque_reverse_sorted(bencher: &mut Bencher)
    {
        let random_values = init_reverse_sorted();
        bencher.iter(|| vecdeque_insertion_sort(&random_values));
    }
}
