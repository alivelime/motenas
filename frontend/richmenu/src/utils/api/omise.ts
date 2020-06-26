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

export function getOmise(env: string, clientId: string, omiseId: string, resolver: (omise: Omise)=>void ) {
  fetcher<ResponseOmise>(
    `${process.env.REACT_APP_API_HOST}/${env}/omise/${clientId}/${omiseId}`,
    {
      method: "GET",
      mode: "cors",
      cache: "no-cache",
    }
  )
    .then(res => {
      res.omise.omotenashi = new Set(res.omise.omotenashi)
      resolver(res.omise)
    })
    .catch(err => alert(err))
}

export function checkOmise(env: string, clientId: string, omiseId: string, charaId: string, accessToken: string) {
  fetcher<void>(
    `${process.env.REACT_APP_LINE_API_HOST}/${env}/line-api/omise/check`,
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
    .catch(err => alert(err))
}
