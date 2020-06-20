function reviver (key: any, value: any) {
  return /\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}/.test(value) ? new Date(value) : value;
}
const wrap = <T>(task: Promise<Response>): Promise<T> => {
  return new Promise((resolve, reject) => {
    task
      .then(response => {
        if (response.ok) {
          response.text()
            .then(text => JSON.parse(text, reviver))
            .then(json => {
              // jsonが取得できた場合だけresolve
              resolve(json)
            })
            .catch(error => {
              reject(error)
            })
        } else {
          reject(response)
        }
      })
      .catch(error => {
        reject(error)
      })
  })
}

const fetcher = <T = any>(
  input: RequestInfo,
  init?: RequestInit
): Promise<T> => {
  return wrap<T>(fetch(input, init))
}

export default fetcher
