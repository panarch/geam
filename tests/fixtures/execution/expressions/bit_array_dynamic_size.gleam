const constant_size = 4

fn dynamic_size() {
  4
}

pub fn main() {
  let local_size = 4
  let value = 0x1234
  let captured_value = 10
  let captured_size = 4
  let captured = fn() { <<captured_value:size(captured_size)>> }

  #(
    <<value:size(local_size)-unit(3)-big>>,
    <<value:size(dynamic_size())-unit(3)-little>>,
    <<1:size({ local_size })>>,
    <<1:size(local_size - 8)>>,
    <<1:size(local_size - 4)>>,
    <<1:size(-1)>>,
    <<1.5:float-size(local_size * 4)-big>>,
    <<1.5:float-size(local_size)-unit(4)-big>>,
    <<1.5:float-size(dynamic_size() * 8)-little>>,
    <<1.5:float-size({ 64 })-big>>,
    <<1:size(constant_size)>>,
    captured(),
  )
}

// geam:expect Tuple([BitArray(bytes=[35, 64], bit_len=12), BitArray(bytes=[52, 32], bit_len=12), BitArray(bytes=[16], bit_len=4), BitArray(bytes=[], bit_len=0), BitArray(bytes=[], bit_len=0), BitArray(bytes=[], bit_len=0), BitArray(bytes=[62, 0], bit_len=16), BitArray(bytes=[62, 0], bit_len=16), BitArray(bytes=[0, 0, 192, 63], bit_len=32), BitArray(bytes=[63, 248, 0, 0, 0, 0, 0, 0], bit_len=64), BitArray(bytes=[16], bit_len=4), BitArray(bytes=[160], bit_len=4)])
