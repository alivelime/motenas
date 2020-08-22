export function isLocal(): boolean {
  return !(process.env.NODE_ENV === "production")
}
export function isDev(): boolean {
  return process.env.REACT_APP_ENV === "dev"
}
export function isPrd(): boolean {
  return process.env.REACT_APP_ENV === "prd"
}

export function LiffId(): string {
  return (process.env.REACT_APP_ENV === "prd"
    ? process.env.REACT_APP_PRD_LIFF_ID
    : process.env.REACT_APP_DEV_LIFF_ID) as string
}
