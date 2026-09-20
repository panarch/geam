import geam_benchmarks/native
import gleam/io
import gleam/json
import gleam/list

pub type Budget {
  Budget(warmup_ns: Int, sample_ns: Int, samples: Int)
}

type Warmup {
  Warmup(iterations: Int, elapsed_ns: Int, total_iterations: Int)
}

type Sample {
  Sample(index: Int, elapsed_ns: Int, checksum: Int)
}

pub fn run(
  workload: String,
  size: Int,
  budget: Budget,
  work: fn() -> a,
  verify: fn(a) -> Int,
) {
  let _ = verify(work())
  let warmup = warm_up(work, budget, native.monotonic_ns(), 1, 0)
  let measurements =
    collect(budget.samples, 1, warmup.iterations, work, verify, [])
  list.each(list.reverse(measurements), fn(sample) {
    json.object([
      #("workload", json.string(workload)),
      #("size", json.int(size)),
      #("iterations", json.int(warmup.iterations)),
      #("warmup_elapsed_ns", json.int(warmup.elapsed_ns)),
      #("warmup_iterations", json.int(warmup.total_iterations)),
      #("sample", json.int(sample.index)),
      #("elapsed_ns", json.int(sample.elapsed_ns)),
      #("checksum", json.int(sample.checksum)),
    ])
    |> json.to_string
    |> io.println
  })
}

fn warm_up(
  work: fn() -> a,
  budget: Budget,
  started: Int,
  iterations: Int,
  total: Int,
) -> Warmup {
  let start = native.monotonic_ns()
  let _ = repeat(iterations, work)
  let end = native.monotonic_ns()
  let next = next_iterations(iterations, end - start, budget.sample_ns)
  case end - started >= budget.warmup_ns {
    True -> Warmup(next, end - started, total + iterations)
    False -> warm_up(work, budget, started, next, total + iterations)
  }
}

pub fn next_iterations(current: Int, elapsed: Int, target: Int) -> Int {
  let elapsed = case elapsed {
    0 -> 1
    n -> n
  }
  let proposed = current * target / elapsed
  let limited = case proposed > current * 8 {
    True -> current * 8
    False -> proposed
  }
  case limited {
    n if n < 1 -> 1
    n if n > 1_000_000 -> 1_000_000
    n -> n
  }
}

fn repeat(remaining: Int, work: fn() -> a) -> a {
  let value = native.consume(work())
  case remaining {
    1 -> value
    _ -> repeat(remaining - 1, work)
  }
}

fn collect(
  remaining: Int,
  index: Int,
  iterations: Int,
  work: fn() -> a,
  verify: fn(a) -> Int,
  samples: List(Sample),
) -> List(Sample) {
  case remaining {
    0 -> samples
    _ -> {
      let start = native.monotonic_ns()
      let result = repeat(iterations, work)
      let elapsed = native.monotonic_ns() - start
      let assert True = elapsed > 0
      let checksum = verify(result)
      collect(remaining - 1, index + 1, iterations, work, verify, [
        Sample(index, elapsed, checksum),
        ..samples
      ])
    }
  }
}
