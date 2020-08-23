import { isDev } from 'utils/env'

export function callbackUserUrl(page: string, clientId: string, omiseId: string): string {
  return `${isDev()
    ? process.env.REACT_APP_DEV_CLOUDFRONT_HOST
    : process.env.REACT_APP_PRD_CLOUDFRONT_HOST
    }/callback?target=user&page=${page}&clientId=${clientId}&omiseId=${omiseId}`
}
export function callbackStaffUrl(page: string, clientId: string, omiseId: string): string {
  return `${isDev()
    ? process.env.REACT_APP_DEV_CLOUDFRONT_HOST
    : process.env.REACT_APP_PRD_CLOUDFRONT_HOST
    }/callback?target=page&page=${page}&clientId=${clientId}&omiseId=${omiseId}`
}

export function cloudfrontUrl(): string {
  return `${isDev()
    ? process.env.REACT_APP_DEV_CLOUDFRONT_HOST
    : process.env.REACT_APP_PRD_CLOUDFRONT_HOST
    }${window.location.pathname}`
}
export function liffUrl(): string {
  return `${isDev()
    ? process.env.REACT_APP_DEV_LIFF_HOST
    : process.env.REACT_APP_PRD_LIFF_HOST
  }${window.location.pathname}`
}
