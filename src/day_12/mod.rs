// tag::setup[]
use crate::Answer;
use std::collections::BTreeMap as Map;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CaveKind {
	Big,
	Small,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Cave<'a> {
	name: &'a str,
	kind: CaveKind,
}

impl<'a> Cave<'a> {
	fn new(name: &'a str) -> Self {
		let is_small = name.chars().map(|c| c.is_ascii_lowercase()).all(|b| b);
		let kind = if is_small {
			CaveKind::Small
		} else {
			CaveKind::Big
		};
		Self { name, kind }
	}
}

#[derive(Debug)]
struct CaveSystem<'a> {
	edges: Map<&'a str, Vec<Cave<'a>>>,
}

impl<'a> CaveSystem<'a> {
	fn from_str(input: &'a str) -> Option<Self> {
		let mut edges = Map::new();
		for line in input.lines() {
			let mut splat = line.split('-');
			let left = splat.next()?;
			let right = splat.next()?;

			for (orig, dest) in [(left, right), (right, left)] {
				if orig != "end" && dest != "start" {
					edges
						.entry(orig)
						.or_insert_with(Vec::new)
						.push(Cave::new(dest));
				}
			}
		}

		Some(Self { edges })
	}
}

impl<'a> CaveSystem<'a> {
	fn traverse_helper(
		&'a self,
		curr_cave: &'a str,
		n_finished: &mut usize,
		cave_visit_counts: &mut Map<&'a str, usize>,
		can_visit_one_small_cave_twice: bool,
		has_visited_a_small_cave_twice: bool,
	) {
		for next_cave in &self.edges[curr_cave] {
			if next_cave.name == "end" {
				*n_finished += 1;
				continue;
			}

			let this_dest_n_visits = cave_visit_counts.entry(next_cave.name).or_insert(0);

			let is_small_cave = next_cave.kind == CaveKind::Small;
			if is_small_cave
				&& (*this_dest_n_visits >= 1
					&& (!can_visit_one_small_cave_twice || has_visited_a_small_cave_twice))
			{
				continue;
			}

			*this_dest_n_visits += 1;
			let n_visits = *this_dest_n_visits;

			self.traverse_helper(
				next_cave.name,
				n_finished,
				cave_visit_counts,
				can_visit_one_small_cave_twice,
				has_visited_a_small_cave_twice || is_small_cave && n_visits >= 2,
			);

			// "un-visit" this cave for the next loop iteration
			cave_visit_counts
				.entry(next_cave.name)
				.and_modify(|v| *v -= 1);
		}
	}

	fn traverse(&'a self, can_visit_one_small_cave_twice: bool) -> usize {
		let mut n_finished = 0;
		let mut cave_visit_counts = Map::new();

		self.traverse_helper(
			"start",
			&mut n_finished,
			&mut cave_visit_counts,
			can_visit_one_small_cave_twice,
			false,
		);

		n_finished
	}
}

fn ans_for_input(input: &str) -> Answer<usize, usize> {
	let cave_system = CaveSystem::from_str(input).unwrap();
	(12, (pt1(&cave_system), pt2(&cave_system))).into()
}

pub fn ans() -> Answer<usize, usize> {
	ans_for_input(include_str!("input.txt"))
}
// end::setup[]

// tag::pt1[]
fn pt1(cave: &CaveSystem) -> usize {
	cave.traverse(false)
}

// end::pt1[]

// tag::pt2[]
fn pt2(cave: &CaveSystem) -> usize {
	cave.traverse(true)
}
// end::pt2[]

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_input;

	#[test]
	fn test() {
		test_input!(include_str!("sample_input_1.txt"), day: 12, ans: (10, 36));
		test_input!(include_str!("sample_input_2.txt"), day: 12, ans: (19, 103));
		test_input!(include_str!("sample_input_3.txt"), day: 12, ans: (226, 3509));
		test_input!(include_str!("input.txt"), day: 12, ans: (3738, 120_506));
	}
}
