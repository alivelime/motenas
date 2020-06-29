import fetcher from 'utils/fetcher'


interface ResponseOmise {
  omise: Omise;
}

export interface Omise {
  clientId: string;
  omiseId: string;
  namae: string;
  url: string;
  yotei: string;
  omotenashi: Set<string>;
  otokoro: Address;

  ima: number,
  hitokoto: "",
  kefuKara: Date;
  kefuMade: Date;
  createdAt: Date;
  updatedAt: Date;
}
export interface Address {
  country: string;
  postcode: number;
  forMap: string;
  todofuken: string;
  prefcode: number;
  city: string;
  street: string;
  building: string;
  access: string;
}

export interface OmiseForm {
  namae: string,
  ima: number,
  hitokoto: string,
  kefuKara: number,
  kefuMade: number,
  omotenashi: Set<string>,
  yotei: string,
  url: string,
  postcode: number,
  prefcode: number,
  city: string,
  street: string,
  building: string,
}

export function newOmise(): Omise {
  return {
    clientId: "",
    omiseId: "",
    namae: "",
    url: "",
    yotei: "",
    omotenashi: new Set<string>([]),
    otokoro: {
      country: "",
      postcode: 1000001,
      forMap: "",
      todofuken: "",
      prefcode: 1,
      city: "",
      street: "",
      building: "",
      access: "",
    },
    ima: 0,
    hitokoto: "",
    kefuKara: new Date(),
    kefuMade: new Date(),
    createdAt: new Date(),
    updatedAt: new Date(),
  };
}

export function getOmise(
  env: string,
  clientId: string,
  omiseId: string,
  resolve: (omise: Omise)=>void,
  reject: (err: Error)=>void,
) {
  fetcher<ResponseOmise>(
    env === 'prd'
      ? `${process.env.REACT_APP_PRD_API_HOST}/omise/${clientId}/${omiseId}`
      : `${process.env.REACT_APP_DEV_API_HOST}/omise/${clientId}/${omiseId}`,
    {
      method: "GET",
      mode: "cors",
      cache: "no-cache",
    }
  )
    .then(res => {
      res.omise.omotenashi = new Set(res.omise.omotenashi)
      resolve(res.omise)
    })
    .catch(err => reject(err))
}

export function setOmise(
  env: string,
  clientId: string,
  omiseId: string,
  charaId: string,
  omise: OmiseForm,
  token: string,
  resolve: ()=>void,
  reject: (err: Error
)=>void) {
  fetcher<void>(
    env === 'prd'
      ? `${process.env.REACT_APP_PRD_LINE_API_HOST}/line-api/omise/set`
      : `${process.env.REACT_APP_DEV_LINE_API_HOST}/line-api/omise/set`,
    {
      method: "POST",
      mode: "cors",
      cache: "no-cache",
      headers: { 'Authorization': 'Bearer: '+token},
      body: JSON.stringify({
        ...omise,
        charaUri: `${clientId}/${omiseId}/${charaId}`,
        postcode: Number(omise.postcode),
        omotenashi: Array.from(omise.omotenashi),
      }),
    }
  )
    .then(()=>resolve())
    .catch(err => reject(err))
}

export function checkOmise(env: string, clientId: string, omiseId: string, charaId: string, accessToken: string) {
  fetcher<void>(
    env === 'prd'
      ? `${process.env.REACT_APP_PRD_LINE_API_HOST}/line-api/omise/check`
      : `${process.env.REACT_APP_DEV_LINE_API_HOST}/line-api/omise/check`,
    {
      method: "POST",
      mode: "cors",
      cache: "no-cache",
      body: JSON.stringify({
        accessToken: accessToken,
        charaId: `${clientId}/${omiseId}/${charaId}`,
      }),
    }
  )
    .then()
    .catch(err => console.log(err))
}
