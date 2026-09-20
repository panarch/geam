import geam_benchmarks/checks
import geam_benchmarks/measurement.{Budget}
import geam_benchmarks/native
import geam_benchmarks/workloads/captures
import geam_benchmarks/workloads/list_inspection
import geam_benchmarks/workloads/lists
import geam_benchmarks/workloads/scalars
import geam_benchmarks/workloads/strings
import geam_benchmarks/workloads/text
import gleam/bit_array
import gleam/int
import gleam/list
import gleam/string

pub fn main() {
  checks.run()
  let assert Ok(workload) = native.environment("GEAM_BENCH_WORKLOAD")
  let assert Ok(raw_size) = native.environment("GEAM_BENCH_SIZE")
  let assert Ok(size) = int.parse(raw_size)
  let assert True = size >= 0
  let budget =
    Budget(
      setting("GEAM_BENCH_WARMUP_MS") * 1_000_000,
      setting("GEAM_BENCH_SAMPLE_MS") * 1_000_000,
      setting("GEAM_BENCH_SAMPLES"),
    )
  case workload {
    "callback_control" -> {
      measurement.run(workload, size, budget, fn() { size }, fn(value) {
        let assert True = value == size
        value
      })
    }
    "count_ones" -> {
      let input = lists.ones_input(size)
      let expected = size / 3 + 1
      measurement.run(
        workload,
        size,
        budget,
        fn() { lists.count_ones(input, 0) },
        fn(value) {
          let assert True = value == expected
          value
        },
      )
    }
    "count_ones_assert" -> {
      let input = lists.ones_input(size)
      let expected = size / 3 + 1
      measurement.run(
        workload,
        size,
        budget,
        fn() { list_inspection.count_ones_assert(input, 0) },
        fn(value) {
          let assert True = value == expected
          value
        },
      )
    }
    "count_ones_equal" -> {
      let input = lists.ones_input(size)
      let stop = []
      let expected = size / 3 + 1
      measurement.run(
        workload,
        size,
        budget,
        fn() { list_inspection.count_ones_equal(input, stop, 0) },
        fn(value) {
          let assert True = value == expected
          value
        },
      )
    }
    "odd_nums_between" -> {
      let expected =
        lists.inclusive_range(size - 1)
        |> list.filter(fn(n) { n % 2 == 1 })
      measurement.run(
        workload,
        size,
        budget,
        fn() { lists.odd_nums_between(0, size, []) },
        fn(value) {
          let assert True = value == expected
          list.fold(value, 0, fn(total, n) { total + n })
        },
      )
    }
    "slice_prefix" | "slice_suffix" -> {
      let input = lists.inclusive_range(10_000)
      let start = case workload {
        "slice_prefix" -> 0
        _ -> 10_000 - size
      }
      let expected = input |> list.drop(start) |> list.take(size)
      measurement.run(
        workload,
        size,
        budget,
        fn() { lists.slice(input, start, start + size, []) },
        fn(value) {
          let assert True = value == expected
          list.fold(value, 0, fn(total, n) { total + n })
        },
      )
    }
    "arithmetic" -> {
      let expected =
        int.range(1, size + 1, 0, fn(total, n) {
          total
          + case n % 2 {
            0 -> n * 3 + 1
            _ -> n * 2 - 1
          }
        })
      measurement.run(
        workload,
        size,
        budget,
        fn() { scalars.arithmetic(size, 0) },
        fn(value) {
          let assert True = value == expected
          value
        },
      )
    }
    "capturing_fold" -> {
      let input = lists.inclusive_range(size - 1)
      let expected = size * { size - 1 } / 2 * 3 + size * 7
      measurement.run(
        workload,
        size,
        budget,
        fn() { scalars.capturing_fold(input, 3, 7) },
        fn(value) {
          let assert True = value == expected
          value
        },
      )
    }
    "capture_chain" -> {
      measurement.run(
        workload,
        size,
        budget,
        fn() { captures.compose(size - 1) },
        fn(value) {
          let assert True = value == size
          value
        },
      )
    }
    "custom_match" -> {
      let input = lists.inclusive_range(size - 1)
      let expected =
        int.range(0, size, 0, fn(total, n) {
          case n % 3 {
            0 -> total + n
            1 -> total - n
            _ -> total
          }
        })
      measurement.run(
        workload,
        size,
        budget,
        fn() { scalars.custom_match(input) },
        fn(value) {
          let assert True = value == expected
          value
        },
      )
    }
    "string_prefixes" -> {
      let input = string.repeat("x", size)
      measurement.run(
        workload,
        size,
        budget,
        fn() { strings.prefixes(input, 0) },
        fn(value) {
          let assert True = value == size
          value
        },
      )
    }
    "string_graphemes" | "string_graphemes_unicode" -> {
      let #(unit, expected) = case workload {
        "string_graphemes" -> #("x", size)
        _ -> #("a\u{301}\u{1f44d}\u{1f3fd}", size * 2)
      }
      let input = string.repeat(unit, size)
      measurement.run(
        workload,
        size,
        budget,
        fn() { strings.graphemes(input, 0) },
        fn(value) {
          let assert True = value == expected
          value
        },
      )
    }
    "string_fields" -> {
      let input = string.repeat(" alpha , beta ,gamma ,", size)
      let expected = string.repeat("alpha|beta|gamma|", size)
      measurement.run(
        workload,
        size,
        budget,
        fn() { text.normalize_fields(input) },
        fn(value) {
          let assert True = value == expected
          string.byte_size(value)
        },
      )
    }
    "bit_checksum" -> {
      let input = bit_array.concat(list.repeat(<<1, 2, 3, 4>>, size))
      measurement.run(
        workload,
        size,
        budget,
        fn() { text.bit_checksum(input, 0) },
        fn(value) {
          let assert True = value == size * 30
          value
        },
      )
    }
    "parse_sum" -> {
      let input = bit_array.from_string(string.repeat("12,7,305,4\n", size))
      measurement.run(
        workload,
        size,
        budget,
        fn() { text.parse_sum(input) },
        fn(value) {
          let assert Ok(total) = value
          let assert True = total == size * 328
          total
        },
      )
    }
    _ -> panic as "unknown benchmark workload"
  }
}

fn setting(name: String) -> Int {
  let assert Ok(raw) = native.environment(name)
  let assert Ok(value) = int.parse(raw)
  let assert True = value > 0
  value
}
