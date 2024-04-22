const isBech32 = (raw) => {
  return !!raw.match(/^zil1[qpzry9x8gf2tvdw0s3jn54khce6mua7l]{38}$/);
};

const CHARSET = "qpzry9x8gf2tvdw0s3jn54khce6mua7l";
const GENERATOR = [0x3b6a57b2, 0x26508e6d, 0x1ea119fa, 0x3d4233dd, 0x2a1462b3];

const isByteString = function (str, len) {
  return !!str.replace('0x', '').match(`^[0-9a-fA-F]{${len}}$`);
}

const isAddress = function (address) {
  return isByteString(address, 40);
}

const convertBits = function(data,fromWidth,toWidth,pad) {
  let acc = 0;
  let bits = 0;
  const ret = [];
  const maxv = (1 << toWidth) - 1;
  for (let p = 0; p < data.length; ++p) {
    const value = data[p];
    if (value < 0 || value >> fromWidth !== 0) {
      return null;
    }
    acc = (acc << fromWidth) | value;
    bits += fromWidth;
    while (bits >= toWidth) {
      bits -= toWidth;
      ret.push((acc >> bits) & maxv);
    }
  }

  if (pad) {
    if (bits > 0) {
      ret.push((acc << (toWidth - bits)) & maxv);
    }
  } else if (bits >= fromWidth || (acc << (toWidth - bits)) & maxv) {
    return null;
  }

  return Buffer.from(ret);
}

const polymod = function (values) {
  let chk = 1;
  for (let p = 0; p < values.length; ++p) {
    const top = chk >> 25;
    chk = ((chk & 0x1ffffff) << 5) ^ values[p];
    for (let i = 0; i < 5; ++i) {
      if ((top >> i) & 1) {
        chk ^= GENERATOR[i];
      }
    }
  }
  return chk;
};

const hrpExpand = function (hrp) {
  const ret = [];
  let p;
  for (p = 0; p < hrp.length; ++p) {
    ret.push(hrp.charCodeAt(p) >> 5);
  }
  ret.push(0);
  for (p = 0; p < hrp.length; ++p) {
    ret.push(hrp.charCodeAt(p) & 31);
  }
  return Buffer.from(ret);
};


function verifyChecksum(hrp, data) {
  return polymod(Buffer.concat([hrpExpand(hrp), data])) === 1;
}

function createChecksum(hrp, data) {
  const values = Buffer.concat([
    Buffer.from(hrpExpand(hrp)),
    data,
    Buffer.from([0, 0, 0, 0, 0, 0]),
  ]);
  // var values = hrpExpand(hrp).concat(data).concat([0, 0, 0, 0, 0, 0]);
  const mod = polymod(values) ^ 1;
  const ret = [];
  for (let p = 0; p < 6; ++p) {
    ret.push((mod >> (5 * (5 - p))) & 31);
  }
  return Buffer.from(ret);
}

const encode = function (hrp, data) {
  const combined = Buffer.concat([data, createChecksum(hrp, data)]);
  let ret = hrp + "1";
  for (let p = 0; p < combined.length; ++p) {
    ret += CHARSET.charAt(combined[p]);
  }
  return ret;
};


const decode = function (bechString) {
  let p;
  let hasLower = false;
  let hasUpper = false;
  for (p = 0; p < bechString.length; ++p) {
    if (bechString.charCodeAt(p) < 33 || bechString.charCodeAt(p) > 126) {
      return null;
    }
    if (bechString.charCodeAt(p) >= 97 && bechString.charCodeAt(p) <= 122) {
      hasLower = true;
    }
    if (bechString.charCodeAt(p) >= 65 && bechString.charCodeAt(p) <= 90) {
      hasUpper = true;
    }
  }
  if (hasLower && hasUpper) {
    return null;
  }
  bechString = bechString.toLowerCase();
  const pos = bechString.lastIndexOf("1");
  if (pos < 1 || pos + 7 > bechString.length || bechString.length > 90) {
    return null;
  }
  const hrp = bechString.substring(0, pos);
  const data = [];
  for (p = pos + 1; p < bechString.length; ++p) {
    const d = CHARSET.indexOf(bechString.charAt(p));
    if (d === -1) {
      return null;
    }
    data.push(d);
  }

  if (!verifyChecksum(hrp, Buffer.from(data))) {
    return null;
  }

  return { hrp, data: Buffer.from(data.slice(0, data.length - 6)) };
};


// HRP is the human-readable part of zilliqa bech32 addresses
const HRP = "zil";


/**
 * toBech32Address
 *
 * Encodes a canonical 20-byte Ethereum-style address as a bech32 zilliqa
 * address.
 *
 * The expected format is zil1<address><checksum> where address and checksum
 * are the result of bech32 encoding a Buffer containing the address bytes.
 *
 * @param {string} 20 byte canonical address
 * @returns {string} 38 char bech32 encoded zilliqa address
 */
const toBech32Address = function (address) {
  if (!isAddress(address)) {
    throw new Error("Invalid address format.");
  }

  const addrBz = convertBits(
    Buffer.from(address.replace("0x", ""), "hex"),
    8,
    5
  );

  if (addrBz === null) {
    throw new Error("Could not convert byte Buffer to 5-bit Buffer");
  }

  return encode(HRP, addrBz);
};

/**
 * fromBech32Address
 *
 * @param {string} address - a valid Zilliqa bech32 address
 * @returns {string} a canonical 20-byte Ethereum-style address
 */
const fromBech32Address = function (address) {
  const res = decode(address);

  if (res === null) {
    throw new Error("Invalid bech32 address");
  }

  const { hrp, data } = res;

  const shouldBe = HRP;
  if (hrp !== shouldBe) {
    throw new Error(`Expected hrp to be ${shouldBe} but got ${hrp}`);
  }

  const buf = convertBits(data, 5, 8, false);

  if (buf === null) {
    throw new Error("Could not convert buffer to bytes");
  }

  return buf.toString("hex");
};

function convertAddress(id) {
  let addressInput = document.getElementById(id);
  let addressValue = addressInput.value;
  try {
    if (isBech32(addressValue)) {
    result = fromBech32Address(addressValue);
  } else {
    result = toBech32Address(addressValue);
  }
  addressInput.value= result;
  } catch (error) {
    alert('Cannot convert ' + error)
  }
}

function copyValue(id) {
  let addressInput = document.getElementById(id);
  addressInput.select();
  addressInput.setSelectionRange(0,99999);
  navigator.clipboard.writeText(addressInput.value);
}

// Fill in dynamically because mkdocs removes onclick handlers.
document$.subscribe(function () {
  var ids = document.getElementsByClassName("hexconverter");
  for (let i = 0;i < ids.length;i++) {
    let elem= ids[i];
    elem.onclick = function() {  convertAddress('address'); }
  }
  var ids2 = document.getElementsByClassName("hexcopy");
  for (let i =0; i< ids2.length;i++) {
    let elem = ids2[i];
    elem.onclick = function () { copyValue('address'); }
  }
})
