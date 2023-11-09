// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

import "hardhat/console.sol";

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
//import "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";

using ECDSA for bytes32;
using ECDSA for bytes; //using MessageHashUtils for bytes;

contract Bridged {
    address relayer;

    function setRelayer(address _relayer) public {
        // TODO: restrict the use of this function
        relayer = _relayer;
    }

    modifier onlyRelayer() {
        require(msg.sender == relayer, "Must be called by relayer");
        _;
    }

    function dispatched(
        address target,
        bytes memory call
    ) public payable onlyRelayer returns (bool success, bytes memory response) {
        console.log("dispatched()");
        (success, response) = target.call{value: msg.value, gas: 100000}(call);
    }

    function queried(
        address target,
        bytes memory call
    ) public view onlyRelayer returns (bool success, bytes memory response) {
        console.log("queried()");
        (success, response) = target.staticcall{gas: 100000}(call);
    }

    function relay(
        address target,
        bytes memory call,
        bool readonly,
        bytes4 callback
    ) internal returns (uint nonce) {
        nonce = Relayer(relayer).relay(target, call, readonly, callback);
    }
}

contract Relayer {
    function getValidators() public view returns (address[] memory validators) {
        validators = new address[](18);
        validators[0] = address(0x70997970C51812dc3A010C7d01b50e0d17dc79C8);
        validators[1] = address(0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC);
        validators[2] = address(0x90F79bf6EB2c4f870365E785982E1f101E93b906);
        validators[3] = address(0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65);
        validators[4] = address(0x9965507D1a55bcC2695C58ba16FB37d819B0A4dc);
        validators[5] = address(0x976EA74026E726554dB657fA54763abd0C3a0aa9);
        validators[6] = address(0x14dC79964da2C08b23698B3D3cc7Ca32193d9955);
        validators[7] = address(0x23618e81E3f5cdF7f54C3d65f7FBc0aBf5B21E8f);
        validators[8] = address(0xa0Ee7A142d267C1f36714E4a8F75612F20a79720);
        validators[9] = address(0xBcd4042DE499D14e55001CcbB24a551F3b954096);
        validators[10] = address(0x71bE63f3384f5fb98995898A86B02Fb2426c5788);
        validators[11] = address(0xFABB0ac9d68B0B445fB7357272Ff202C5651694a);
        validators[12] = address(0x1CBd3b2770909D4e10f157cABC84C7264073C9Ec);
        validators[13] = address(0xdF3e18d64BC6A983f673Ab319CCaE4f1a57C7097);
        validators[14] = address(0xcd3B766CCDd6AE721141F452C550Ca635964ce71);
        validators[15] = address(0x2546BcD3c84621e976D8185a91A922aE77ECEc30);
        validators[16] = address(0xbDA5747bFD65F08deb54cb465eB87D40e51B197E);
        validators[17] = address(0xdD2FD4581271e230360230F9337D5c0430Bf44C0);
        /*	validators[18] = address(0x8626f6940E2eb28930eFb4CeF49B2d1F2C9C1199);
         */
    }

    mapping(address => uint) nonces;
    mapping(address => mapping(uint => bool)) dispatched;
    mapping(address => mapping(uint => bool)) resumed;

    event Relayed(
        address caller,
        address target,
        bytes call,
        bool readonly,
        bytes4 callback,
        uint nonce
    );

    function relay(
        address target,
        bytes memory call,
        bool readonly,
        bytes4 callback
    ) public returns (uint) {
        emit Relayed(
            msg.sender,
            target,
            call,
            readonly,
            callback,
            nonces[msg.sender]
        );
        nonces[msg.sender]++;
        return nonces[msg.sender];
    }

    event Dispatched(
        address indexed caller,
        bytes4 callback,
        bool success,
        bytes response,
        uint indexed nonce
    );

    function dispatch(
        address caller,
        address target,
        bytes memory call,
        bytes4 callback,
        uint nonce,
        uint16[] memory indices,
        bytes[] memory signatures
    ) public {
        require(!dispatched[caller][nonce], "Already dispatched");
        address[] memory validators = getValidators();
        require(3 * indices.length > 2 * validators.length, "No supermajority");
        bytes memory message = abi.encode(
            caller,
            target,
            call,
            false,
            callback,
            nonce
        );
        bytes32 hash = message.toEthSignedMessageHash();
        for (uint i = 0; i < signatures.length; i++) {
            require(i == 0 || indices[i] > indices[i - 1], "Wrong index");
            address signer = hash.recover(signatures[i]);
            require(signer == validators[indices[i]], "Wrong validator");
        }
        require(caller.code.length > 0);
        (bool success, bytes memory response) = Bridged(caller).dispatched(
            target,
            call
        );
        emit Dispatched(caller, callback, success, response, nonce);
        dispatched[caller][nonce] = true;
    }

    function query(
        address caller,
        address target,
        bytes memory call
    ) public view returns (bool success, bytes memory response) {
        require(caller.code.length > 0);
        (success, response) = Bridged(caller).queried(target, call);
    }

    event Resumed(
        address indexed caller,
        bytes call,
        bool success,
        bytes response,
        uint indexed nonce
    );

    function resume(
        address caller,
        bytes4 callback,
        bool success,
        bytes memory response,
        uint nonce,
        uint16[] memory indices,
        bytes[] memory signatures
    ) public payable {
        require(!resumed[caller][nonce], "Already resumed");
        address[] memory validators = getValidators();
        require(3 * indices.length > 2 * validators.length, "No supermajority");
        bytes memory message = abi.encode(
            caller,
            callback,
            success,
            response,
            nonce
        );
        bytes32 hash = message.toEthSignedMessageHash();
        for (uint i = 0; i < signatures.length; i++) {
            require(i == 0 || indices[i] > indices[i - 1], "Wrong index");
            address signer = hash.recover(signatures[i]);
            require(signer == validators[indices[i]], "Wrong validator");
        }
        bytes memory call = abi.encodeWithSelector(
            callback,
            success,
            response,
            nonce
        );
        (bool success2, bytes memory response2) = caller.call{
            value: msg.value,
            gas: 100000
        }(call);
        emit Resumed(caller, call, success2, response2, nonce);
        resumed[caller][nonce] = true;
    }
}

contract CollectorRelayer is Relayer {
    event Echoed(bytes32 indexed hash, uint16 index, bytes signature);

    function echo(bytes32 hash, uint16 index, bytes memory signature) public {
        address[] memory validators = getValidators();
        address signer = hash.recover(signature);
        require(signer == validators[index], "Wrong validator");
        emit Echoed(hash, index, signature);
    }
}
