function convertBinaryToDecimal (binary) {
  return new Promise((resolve, reject) => {
    const regExpNotBin = /[^01]/
    if (regExpNotBin.test(binary)) reject(new Error(`This is not binary: ${binary} ${typeof binary}`))
    let result = 0
    let j = 0
    for (let i = binary.length - 1; i >= 0; i--) {
      const element = binary[i]
      result += element * 2 ** j++
    }
    resolve(result)
  })
}

function convertDecimalToBinary (decimal) {
  return new Promise((resolve, reject) => {
    const regExpNotDec = /^[0-9]$/
    if (regExpNotDec.test(decimal)) reject(new Error(`This is not number: ${decimal}`))

    let binary = ''
    let resultDivision = decimal
    let resultResiduio = decimal

    while (resultDivision > 1) {
      resultResiduio = resultDivision % 2
      resultDivision = resultDivision / 2
      binary = parseInt(resultResiduio) + binary
    }
    resolve(binary)
  })
}

export { convertBinaryToDecimal, convertDecimalToBinary }
