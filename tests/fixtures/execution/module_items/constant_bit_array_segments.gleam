const values = #(
  <<>>,
  <<1:size(0)>>,
  <<0x1234:size(12)-big>>,
  <<0x1234:size(12)-little>>,
  <<-1:size(4)>>,
  <<1:size(2)-unit(4)>>,
  <<1.5:float-size(16)-big>>,
  <<1.5:float-size(16)-little>>,
  <<1.5:float-size(32)-big>>,
  <<1.5:float-size(32)-little>>,
  <<1.5:float-size(64)-big>>,
  <<1.5:float-size(64)-little>>,
  <<"안">>,
  <<"안":utf8>>,
  <<"안":utf16-big>>,
  <<"안":utf16-little>>,
  <<"A":utf32-big>>,
  <<"A":utf32-little>>,
  <<1:size(4), <<2:size(4)>>:bits>>,
)

pub fn main() {
  values
}

// geam:expect Tuple([BitArray(bytes=[], bit_len=0), BitArray(bytes=[], bit_len=0), BitArray(bytes=[35, 64], bit_len=12), BitArray(bytes=[52, 32], bit_len=12), BitArray(bytes=[240], bit_len=4), BitArray(bytes=[1], bit_len=8), BitArray(bytes=[62, 0], bit_len=16), BitArray(bytes=[0, 62], bit_len=16), BitArray(bytes=[63, 192, 0, 0], bit_len=32), BitArray(bytes=[0, 0, 192, 63], bit_len=32), BitArray(bytes=[63, 248, 0, 0, 0, 0, 0, 0], bit_len=64), BitArray(bytes=[0, 0, 0, 0, 0, 0, 248, 63], bit_len=64), BitArray(bytes=[236, 149, 136], bit_len=24), BitArray(bytes=[236, 149, 136], bit_len=24), BitArray(bytes=[197, 72], bit_len=16), BitArray(bytes=[72, 197], bit_len=16), BitArray(bytes=[0, 0, 0, 65], bit_len=32), BitArray(bytes=[65, 0, 0, 0], bit_len=32), BitArray(bytes=[18], bit_len=8)])
