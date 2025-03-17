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
};

export enum RecvEventType {
  NewSkin = "newSkin",
  NewSpray = "newSpray",
};

export interface NewFileEvent {
  id: SendEventType.NewFile,
  data: File,
};

export interface NewSprayEvent {
  id: RecvEventType.NewSpray,
  data: Spray,
};

export type SendEvent = NewFileEvent;
export type RecvEvent = NewSprayEvent;
