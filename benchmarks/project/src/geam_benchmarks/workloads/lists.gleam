// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2025 The Gleam contributors
// Adapted from gleam-lang/gleam, commit 19bf207ebb7d953ea1391f041da48c214ee1440a,
// benchmark/list/test/list_test.gleam. Algorithm and inclusive range preserved.

import gleam/int
import gleam/list

pub fn ones_input(size: Int) -> List(Int) {
  inclusive_range(size)
  |> list.map(fn(n) {
    case n {
      _ if n % 3 == 0 -> 1
      _ -> 0
    }
  })
}

pub fn count_ones(values: List(Int), count: Int) -> Int {
  case values {
    [] -> count
    [1, ..tail] -> count_ones(tail, count + 1)
    [_, ..tail] -> count_ones(tail, count)
  }
}

pub fn inclusive_range(size: Int) -> List(Int) {
  int.range(from: size, to: -1, with: [], run: fn(values, n) { [n, ..values] })
}

pub fn odd_nums_between(start: Int, end: Int, acc: List(Int)) -> List(Int) {
  case start {
    start if start >= end -> list.reverse(acc)
    _ -> {
      let acc = case start {
        n if n % 2 == 1 -> [n, ..acc]
        _ -> acc
      }
      odd_nums_between(start + 1, end, acc)
    }
  }
}

pub fn slice(
  values: List(Int),
  start: Int,
  end: Int,
  acc: List(Int),
) -> List(Int) {
  case values {
    [_, ..tail] if start > 0 -> slice(tail, start - 1, end - 1, acc)
    [head, ..tail] if end > 0 -> slice(tail, start - 1, end - 1, [head, ..acc])
    _ -> list.reverse(acc)
  }
}
