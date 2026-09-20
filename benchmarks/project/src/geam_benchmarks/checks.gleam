import geam_benchmarks/measurement
import geam_benchmarks/native
import geam_benchmarks/workloads/captures
import geam_benchmarks/workloads/list_inspection
import geam_benchmarks/workloads/lists
import geam_benchmarks/workloads/scalars
import geam_benchmarks/workloads/strings
import geam_benchmarks/workloads/text

pub fn run() {
  let assert [1, 0, 0, 1, 0, 0, 1] = lists.ones_input(6)
  let assert 3 = lists.count_ones([1, 0, 1, 0, 1], 0)
  let assert 0 = list_inspection.count_ones_assert([], 0)
  let assert 1 = list_inspection.count_ones_assert([1], 0)
  let assert 0 = list_inspection.count_ones_assert([0], 0)
  let assert 5 = list_inspection.count_ones_assert([1, 0, 1, 0, 1], 2)
  let assert 0 = list_inspection.count_ones_equal([], [], 0)
  let assert 1 = list_inspection.count_ones_equal([1], [], 0)
  let assert 0 = list_inspection.count_ones_equal([0], [], 0)
  let assert 5 = list_inspection.count_ones_equal([1, 0, 1, 0, 1], [], 2)
  let assert 1 = list_inspection.count_ones_equal([1, 0, 1], [0, 1], 0)
  let assert 2 = list_inspection.count_ones_equal([], [1], 2)
  let assert [1, 3, 5] = lists.odd_nums_between(0, 6, [])
  let assert [2, 3] = lists.slice([0, 1, 2, 3, 4], 2, 4, [])
  let assert [] = lists.slice([0, 1], 0, 0, [])
  let assert 0 = scalars.arithmetic(0, 0)
  let assert 26 = scalars.arithmetic(4, 0)
  let assert 30 = scalars.capturing_fold([0, 1, 2], 3, 7)
  let assert 1 = captures.compose(0)
  let assert 2 = captures.compose(1)
  let assert 9 = captures.compose(8)
  let assert -2 = scalars.custom_match([0, 1, 2, 3, 4, 5])
  let assert 0 = strings.prefixes("", 0)
  let assert 1 = strings.prefixes("x", 0)
  let assert 10 = strings.prefixes("xxxxxxxx", 2)
  let assert 0 = strings.graphemes("", 0)
  let assert 1 = strings.graphemes("x", 0)
  let assert 8 = strings.graphemes("xxxxxxxx", 0)
  let assert 2 = strings.graphemes("a\u{301}\u{1f44d}\u{1f3fd}", 0)
  let assert 2 = strings.graphemes("\u{1f1e6}\u{1f1e7}\u{1f1e8}", 0)
  let assert "alpha|beta|gamma|" =
    text.normalize_fields(" alpha , beta ,gamma ,")
  let assert 30 = text.bit_checksum(<<1, 2, 3, 4>>, 0)
  let assert Ok(328) = text.parse_sum(<<"12,7,305,4\n":utf8>>)
  let assert Ok(12) = text.parse_sum(<<"12":utf8>>)
  let assert Error(Nil) = text.parse_sum(<<"x":utf8>>)
  let assert 80 = measurement.next_iterations(10, 100, 1000)
  let assert 1 = measurement.next_iterations(10, 100, 1)
  let assert 10 = measurement.next_iterations(10, 100, 100)
  let assert 8 = measurement.next_iterations(1, 0, 100)
  let assert 1_000_000 = measurement.next_iterations(1_000_000, 1, 100)
  let assert [1, 2, 3] = native.consume([1, 2, 3])
  let assert <<1, 2>> = native.consume(<<1, 2>>)
  let assert "checked" = native.consume("checked")
  let assert Ok(3) = native.consume(Ok(3))
  Nil
}
