export type Spray = {
  id: string,
  name: string,
};

export type Skin = {
  name: string,
  realname: string,
  kartspeed: number,
  kartweight: number,
};

/* Worker communication events */
export enum SendEventType {
  NewFile = "newFile",
  GenerateSprayImage = "generateSprayImage",
};

export enum RecvEventType {
  NewSkin = "newSkin",
  NewSpray = "newSpray",
  GenerateSprayImage = "generateSprayImage",
  Error = "error",
};

export interface NewFileEvent {
  id: SendEventType.NewFile,
  data: File,
};

export interface NewSprayEvent {
  id: RecvEventType.NewSpray,
  data: Spray,
};

export interface GenerateSpraySendEvent {
  id: SendEventType.GenerateSprayImage,
  data: string,
  seq: number,
}

export interface GenerateSprayRecvEvent {
  id: RecvEventType.GenerateSprayImage,
  data: string,
  seq: number,
}

export interface ErrorEvent {
  id: RecvEventType.Error,
  data: any,
  seq: number,
}

export type SendEvent = NewFileEvent | GenerateSpraySendEvent;
export type RecvEvent = NewSprayEvent | GenerateSprayRecvEvent | ErrorEvent;
