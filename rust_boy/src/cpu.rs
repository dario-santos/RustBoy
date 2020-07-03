// Todo: CPU
//  - Flags
//  - Registers
//  - Instructions

// Registers
// 
// | Register | High | Low | Name/Functionality |
// +:--------:+------+-----+--------------------+
// |    AF    |  A   |  -  | Accumulator, Flags |
// |    BC    |  B   |  C  | BC                 |
// |    DE    |  D   |  E  | DE                 |
// |    HL    |  H   |  L  | HL                 |
// |    SP    |  -   |  -  | Stack Pointer      |
// |    PC    |  -   |  -  | Program Counter    |


// The Flag Registor (The lower 8 bits of AF)
//
// | Bit | Name | Set | Clr | Explanation |
// +-----+------+-----+-----+-------------+
// |  7  |  zf  |  Z  |     |
// |  6  |  n   |  -  |     |
// |  5  |  h   |  -  |     |
// |  4  |  cy  |  C  |     |
// | 0-3 |  -   |  -  |     |
