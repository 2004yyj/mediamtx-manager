export interface GlobalConfig {
  logLevel?: string;
  logDestinations?: string[];
  readTimeout?: string;
  writeTimeout?: string;
  readBufferCount?: number;
  api?: boolean;
  apiAddress?: string;
  rtsp?: boolean;
  rtspAddress?: string;
  rtmp?: boolean;
  rtmpAddress?: string;
  hls?: boolean;
  hlsAddress?: string;
  webrtc?: boolean;
  webrtcAddress?: string;
  srt?: boolean;
  srtAddress?: string;
  metrics?: boolean;
  metricsAddress?: string;
  record?: boolean;
  recordPath?: string;
  recordFormat?: string;
  [key: string]: unknown;
}

export interface PathConfig {
  name?: string;
  source?: string;
  sourceOnDemand?: boolean;
  sourceOnDemandStartTimeout?: string;
  sourceOnDemandCloseAfter?: string;
  record?: boolean;
  recordPath?: string;
  recordFormat?: string;
  runOnInit?: string;
  runOnReady?: string;
  runOnRead?: string;
  runOnUnread?: string;
  [key: string]: unknown;
}

export interface PathConfigList {
  pageCount: number;
  items: PathConfig[];
}

export interface SourceInfo {
  type?: string;
  id?: string;
}

export interface ReaderInfo {
  type?: string;
  id?: string;
}

export interface PathItem {
  name: string;
  source?: SourceInfo;
  readers?: ReaderInfo[];
  ready?: boolean;
  bytesReceived?: number;
  bytesSent?: number;
}

export interface PathList {
  pageCount: number;
  items: PathItem[];
}

export type ProcessStatusType = "stopped" | "starting" | "running" | "stopping" | "error";

export interface ProcessStatus {
  status: ProcessStatusType;
  message?: string;
}

export interface BinaryInfo {
  version: string;
  path: string;
  os: string;
  arch: string;
}
