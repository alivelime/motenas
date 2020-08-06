/* eslint no-restricted-globals: ["error"] */
export default () => {
  self.jsQR =  require('jsqr')
  self.Encoding =  require('encoding-japanese')
  self.addEventListener('message', function(e) {
    var decoded = self.jsQR(
      e.data.data,
      e.data.width,
      e.data.height,
      {inversionAttempts: 'onlyInvert'}
    )
    if (decoded) {
      if (decoded.data != null) {
        postMessage(decoded.data)
      } else if (decoded.binaryData != null) {
        postMessage(self.Encoding.codeToString(self.Encoding.convert(decoded.binaryData, 'UNICODE', 'SJIS')))
      }
    } else {
      postMessage(null)
    }
  })
}
