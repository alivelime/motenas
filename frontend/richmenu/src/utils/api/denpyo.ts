import fetcher from 'utils/fetcher'
import { isDev } from 'utils/env'

export interface Denpyo {
  clientId: string,
  omiseId: string,
  id: string,
}

export function enterOmise(
  clientId: string,
  omiseId: string,
  id: string,
  token: string | null,
  resolve: ()=>void,
  reject: (err: Error)=>void,
) {
  fetcher<void>(
    isDev()
      ? `${process.env.REACT_APP_DEV_LINE_API_HOST}/line-api/omise/enter`
      : `${process.env.REACT_APP_PRD_LINE_API_HOST}/line-api/omise/enter`,
    {
      method: "POST",
      mode: "cors",
      cache: "no-cache",
      headers: { 'Authorization': 'Bearer: '+token},
      body: JSON.stringify({
        clientId: clientId,
        omiseId: omiseId,
        id: id,
      }),
    }
  )
  .then(()=>resolve())
  .catch(err => reject(err))
}
