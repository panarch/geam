pub fn main() {
  #(
    <<1.5:float-size(16)-big>>,
    <<1.5:float-size(16)-little>>,
  )
}

// geam:expect Tuple([BitArray(bytes=[62, 0], bit_len=16), BitArray(bytes=[0, 62], bit_len=16)])
