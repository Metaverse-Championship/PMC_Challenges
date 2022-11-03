// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;
import "@openzeppelin/contracts/utils/Strings.sol";

/// @title On-chain storage to save a precious memory of you life
/// @author Challenge inspired by CCTF, coded by six
/// @notice The flag should return when the data is on-chain.
contract PMC_Sol_Challenge {
    // Declare everything needed in the contract
    address private admin = 0x5B38Da6a701c568545dCfcB03FcB875f56beddC4;
    uint160 door = 0;
    event contractStart(address indexed _admin);
    uint256 public pin;
    string private start;
    string private data;
    string private end;
    string private flag;
    string private result;
    bytes32 private data_store;
    uint private data_set;
    string private ds_to_Str;

    // O, that is nasty!
    constructor(address O) payable {
        emit contractStart(admin);
        admin = 0==0?O:0x0000000000000000000000000000000000000000;
        pin = block.number + block.timestamp;  // "Random" pin
        start = "PMC{";
        data_set = 0;
        end = ".NiceTry}";
    }

    /// @notice Mod
    function callAddMod() public pure returns(uint){
       return addmod(4, 5, 3);
    }

    /// @notice Only the central admin should be able to store his life memory in bytes32.
    function store_me_on_chain(bytes32 _data) public returns (string memory) {
        require(msg.sender == admin, 'Not admin');
        data_store = _data;
        data_set = asmbl();
        ds_to_Str = Strings.toString(data_set);
        return data = string("den_who_is_dan_");
    }

    /// @notice Generate number
    function asmbl() public view returns (uint8 b) {
        assembly {
            let c := add(27, 16)
            mstore(0x80, c)
            { let d := add(sload(c), 255) b := d }
            b := add(b, c) }
        }

    /// @notice The admin wants to check the data is on chain and return the flag if is!
    function is_it_on_chain(uint256 _pin) external returns (string memory) {
        require(msg.sender == admin, 'Not admin');
        require(pin == _pin, 'Wrong pin');
        require(data_set > 1, 'Nothing in the on-chain storage');
        flag = string.concat(start,data,ds_to_Str,end);
        return flag;
        }

    fallback() external {}
}
