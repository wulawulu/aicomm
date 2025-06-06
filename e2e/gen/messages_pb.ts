// @generated by protoc-gen-es v2.4.0 with parameter "target=ts"
// @generated from file messages.proto (package analytics, syntax proto3)
/* eslint-disable */

import type { GenEnum, GenFile, GenMessage } from "@bufbuild/protobuf/codegenv1";
import { enumDesc, fileDesc, messageDesc } from "@bufbuild/protobuf/codegenv1";
import type { Message } from "@bufbuild/protobuf";

/**
 * Describes the file messages.proto.
 */
export const file_messages: GenFile = /*@__PURE__*/
  fileDesc("Cg5tZXNzYWdlcy5wcm90bxIJYW5hbHl0aWNzIr0ECg5BbmFseXRpY3NFdmVudBIoCgdjb250ZXh0GAEgASgLMhcuYW5hbHl0aWNzLkV2ZW50Q29udGV4dBItCglhcHBfc3RhcnQYCCABKAsyGC5hbmFseXRpY3MuQXBwU3RhcnRFdmVudEgAEisKCGFwcF9leGl0GAkgASgLMhcuYW5hbHl0aWNzLkFwcEV4aXRFdmVudEgAEi8KCnVzZXJfbG9naW4YCiABKAsyGS5hbmFseXRpY3MuVXNlckxvZ2luRXZlbnRIABIxCgt1c2VyX2xvZ291dBgLIAEoCzIaLmFuYWx5dGljcy5Vc2VyTG9nb3V0RXZlbnRIABI1Cg11c2VyX3JlZ2lzdGVyGAwgASgLMhwuYW5hbHl0aWNzLlVzZXJSZWdpc3RlckV2ZW50SAASMwoMY2hhdF9jcmVhdGVkGA0gASgLMhsuYW5hbHl0aWNzLkNoYXRDcmVhdGVkRXZlbnRIABIzCgxtZXNzYWdlX3NlbnQYDiABKAsyGy5hbmFseXRpY3MuTWVzc2FnZVNlbnRFdmVudEgAEjEKC2NoYXRfam9pbmVkGA8gASgLMhouYW5hbHl0aWNzLkNoYXRKb2luZWRFdmVudEgAEi0KCWNoYXRfbGVmdBgQIAEoCzIYLmFuYWx5dGljcy5DaGF0TGVmdEV2ZW50SAASMAoKbmF2aWdhdGlvbhgRIAEoCzIaLmFuYWx5dGljcy5OYXZpZ2F0aW9uRXZlbnRIAEIMCgpldmVudF90eXBlIg8KDUFwcFN0YXJ0RXZlbnQimAEKDEFwcEV4aXRFdmVudBIzCglleGl0X2NvZGUYASABKA4yIC5hbmFseXRpY3MuQXBwRXhpdEV2ZW50LkV4aXRDb2RlIlMKCEV4aXRDb2RlEhkKFUVYSVRfQ09ERV9VTlNQRUNJRklFRBAAEhUKEUVYSVRfQ09ERV9TVUNDRVNTEAESFQoRRVhJVF9DT0RFX0ZBSUxVUkUQAiIfCg5Vc2VyTG9naW5FdmVudBINCgVlbWFpbBgBIAEoCSIgCg9Vc2VyTG9nb3V0RXZlbnQSDQoFZW1haWwYASABKAkiOAoRVXNlclJlZ2lzdGVyRXZlbnQSDQoFZW1haWwYASABKAkSFAoMd29ya3NwYWNlX2lkGAIgASgJIigKEENoYXRDcmVhdGVkRXZlbnQSFAoMd29ya3NwYWNlX2lkGAEgASgJIlQKEE1lc3NhZ2VTZW50RXZlbnQSDwoHY2hhdF9pZBgBIAEoCRIMCgR0eXBlGAIgASgJEgwKBHNpemUYAyABKAUSEwoLdG90YWxfZmlsZXMYBCABKAUiIgoPQ2hhdEpvaW5lZEV2ZW50Eg8KB2NoYXRfaWQYASABKAkiIAoNQ2hhdExlZnRFdmVudBIPCgdjaGF0X2lkGAEgASgJIisKD05hdmlnYXRpb25FdmVudBIMCgRmcm9tGAEgASgJEgoKAnRvGAIgASgJItkBCgxFdmVudENvbnRleHQSEQoJY2xpZW50X2lkGAEgASgJEhMKC2FwcF92ZXJzaW9uGAIgASgJEiUKBnN5c3RlbRgDIAEoCzIVLmFuYWx5dGljcy5TeXN0ZW1JbmZvEg8KB3VzZXJfaWQYBCABKAkSCgoCaXAYBSABKAkSEgoKdXNlcl9hZ2VudBgGIAEoCRIjCgNnZW8YByABKAsyFi5hbmFseXRpY3MuR2VvTG9jYXRpb24SEQoJY2xpZW50X3RzGAggASgDEhEKCXNlcnZlcl90cxgJIAEoAyJICgpTeXN0ZW1JbmZvEgoKAm9zGAEgASgJEgwKBGFyY2gYAiABKAkSDgoGbG9jYWxlGAMgASgJEhAKCHRpbWV6b25lGAQgASgJIjwKC0dlb0xvY2F0aW9uEg8KB2NvdW50cnkYASABKAkSDgoGcmVnaW9uGAIgASgJEgwKBGNpdHkYAyABKAliBnByb3RvMw");

/**
 * / 用户事件
 *
 * @generated from message analytics.AnalyticsEvent
 */
export type AnalyticsEvent = Message<"analytics.AnalyticsEvent"> & {
  /**
   * @generated from field: analytics.EventContext context = 1;
   */
  context?: EventContext;

  /**
   * @generated from oneof analytics.AnalyticsEvent.event_type
   */
  eventType: {
    /**
     * @generated from field: analytics.AppStartEvent app_start = 8;
     */
    value: AppStartEvent;
    case: "appStart";
  } | {
    /**
     * @generated from field: analytics.AppExitEvent app_exit = 9;
     */
    value: AppExitEvent;
    case: "appExit";
  } | {
    /**
     * @generated from field: analytics.UserLoginEvent user_login = 10;
     */
    value: UserLoginEvent;
    case: "userLogin";
  } | {
    /**
     * @generated from field: analytics.UserLogoutEvent user_logout = 11;
     */
    value: UserLogoutEvent;
    case: "userLogout";
  } | {
    /**
     * @generated from field: analytics.UserRegisterEvent user_register = 12;
     */
    value: UserRegisterEvent;
    case: "userRegister";
  } | {
    /**
     * @generated from field: analytics.ChatCreatedEvent chat_created = 13;
     */
    value: ChatCreatedEvent;
    case: "chatCreated";
  } | {
    /**
     * @generated from field: analytics.MessageSentEvent message_sent = 14;
     */
    value: MessageSentEvent;
    case: "messageSent";
  } | {
    /**
     * @generated from field: analytics.ChatJoinedEvent chat_joined = 15;
     */
    value: ChatJoinedEvent;
    case: "chatJoined";
  } | {
    /**
     * @generated from field: analytics.ChatLeftEvent chat_left = 16;
     */
    value: ChatLeftEvent;
    case: "chatLeft";
  } | {
    /**
     * @generated from field: analytics.NavigationEvent navigation = 17;
     */
    value: NavigationEvent;
    case: "navigation";
  } | { case: undefined; value?: undefined };
};

/**
 * Describes the message analytics.AnalyticsEvent.
 * Use `create(AnalyticsEventSchema)` to create a new message.
 */
export const AnalyticsEventSchema: GenMessage<AnalyticsEvent> = /*@__PURE__*/
  messageDesc(file_messages, 0);

/**
 * / 应用启动事件
 *
 * @generated from message analytics.AppStartEvent
 */
export type AppStartEvent = Message<"analytics.AppStartEvent"> & {
};

/**
 * Describes the message analytics.AppStartEvent.
 * Use `create(AppStartEventSchema)` to create a new message.
 */
export const AppStartEventSchema: GenMessage<AppStartEvent> = /*@__PURE__*/
  messageDesc(file_messages, 1);

/**
 * / 应用退出事件
 *
 * @generated from message analytics.AppExitEvent
 */
export type AppExitEvent = Message<"analytics.AppExitEvent"> & {
  /**
   * @generated from field: analytics.AppExitEvent.ExitCode exit_code = 1;
   */
  exitCode: AppExitEvent_ExitCode;
};

/**
 * Describes the message analytics.AppExitEvent.
 * Use `create(AppExitEventSchema)` to create a new message.
 */
export const AppExitEventSchema: GenMessage<AppExitEvent> = /*@__PURE__*/
  messageDesc(file_messages, 2);

/**
 * @generated from enum analytics.AppExitEvent.ExitCode
 */
export enum AppExitEvent_ExitCode {
  /**
   * @generated from enum value: EXIT_CODE_UNSPECIFIED = 0;
   */
  UNSPECIFIED = 0,

  /**
   * @generated from enum value: EXIT_CODE_SUCCESS = 1;
   */
  SUCCESS = 1,

  /**
   * @generated from enum value: EXIT_CODE_FAILURE = 2;
   */
  FAILURE = 2,
}

/**
 * Describes the enum analytics.AppExitEvent.ExitCode.
 */
export const AppExitEvent_ExitCodeSchema: GenEnum<AppExitEvent_ExitCode> = /*@__PURE__*/
  enumDesc(file_messages, 2, 0);

/**
 * / 用户登录事件
 *
 * @generated from message analytics.UserLoginEvent
 */
export type UserLoginEvent = Message<"analytics.UserLoginEvent"> & {
  /**
   * / 用户邮箱(Email 是 PII，需要脱敏)
   *
   * @generated from field: string email = 1;
   */
  email: string;
};

/**
 * Describes the message analytics.UserLoginEvent.
 * Use `create(UserLoginEventSchema)` to create a new message.
 */
export const UserLoginEventSchema: GenMessage<UserLoginEvent> = /*@__PURE__*/
  messageDesc(file_messages, 3);

/**
 * / 用户登出事件
 *
 * @generated from message analytics.UserLogoutEvent
 */
export type UserLogoutEvent = Message<"analytics.UserLogoutEvent"> & {
  /**
   * / 用户邮箱(Email 是 PII，需要脱敏)
   *
   * @generated from field: string email = 1;
   */
  email: string;
};

/**
 * Describes the message analytics.UserLogoutEvent.
 * Use `create(UserLogoutEventSchema)` to create a new message.
 */
export const UserLogoutEventSchema: GenMessage<UserLogoutEvent> = /*@__PURE__*/
  messageDesc(file_messages, 4);

/**
 * / 用户注册事件
 *
 * @generated from message analytics.UserRegisterEvent
 */
export type UserRegisterEvent = Message<"analytics.UserRegisterEvent"> & {
  /**
   * / 用户邮箱(Email 是 PII，需要脱敏)
   *
   * @generated from field: string email = 1;
   */
  email: string;

  /**
   * / 工作空间 ID
   *
   * @generated from field: string workspace_id = 2;
   */
  workspaceId: string;
};

/**
 * Describes the message analytics.UserRegisterEvent.
 * Use `create(UserRegisterEventSchema)` to create a new message.
 */
export const UserRegisterEventSchema: GenMessage<UserRegisterEvent> = /*@__PURE__*/
  messageDesc(file_messages, 5);

/**
 * / chat 创建事件
 *
 * @generated from message analytics.ChatCreatedEvent
 */
export type ChatCreatedEvent = Message<"analytics.ChatCreatedEvent"> & {
  /**
   * / 工作空间 ID
   *
   * @generated from field: string workspace_id = 1;
   */
  workspaceId: string;
};

/**
 * Describes the message analytics.ChatCreatedEvent.
 * Use `create(ChatCreatedEventSchema)` to create a new message.
 */
export const ChatCreatedEventSchema: GenMessage<ChatCreatedEvent> = /*@__PURE__*/
  messageDesc(file_messages, 6);

/**
 * / 消息发送事件
 *
 * @generated from message analytics.MessageSentEvent
 */
export type MessageSentEvent = Message<"analytics.MessageSentEvent"> & {
  /**
   * / chat ID
   *
   * @generated from field: string chat_id = 1;
   */
  chatId: string;

  /**
   * / 消息类型
   *
   * @generated from field: string type = 2;
   */
  type: string;

  /**
   * / 消息大小
   *
   * @generated from field: int32 size = 3;
   */
  size: number;

  /**
   * / 附件数量
   *
   * @generated from field: int32 total_files = 4;
   */
  totalFiles: number;
};

/**
 * Describes the message analytics.MessageSentEvent.
 * Use `create(MessageSentEventSchema)` to create a new message.
 */
export const MessageSentEventSchema: GenMessage<MessageSentEvent> = /*@__PURE__*/
  messageDesc(file_messages, 7);

/**
 * / 加入聊天事件
 *
 * @generated from message analytics.ChatJoinedEvent
 */
export type ChatJoinedEvent = Message<"analytics.ChatJoinedEvent"> & {
  /**
   * / chat ID
   *
   * @generated from field: string chat_id = 1;
   */
  chatId: string;
};

/**
 * Describes the message analytics.ChatJoinedEvent.
 * Use `create(ChatJoinedEventSchema)` to create a new message.
 */
export const ChatJoinedEventSchema: GenMessage<ChatJoinedEvent> = /*@__PURE__*/
  messageDesc(file_messages, 8);

/**
 * / 离开聊天事件
 *
 * @generated from message analytics.ChatLeftEvent
 */
export type ChatLeftEvent = Message<"analytics.ChatLeftEvent"> & {
  /**
   * / chat ID
   *
   * @generated from field: string chat_id = 1;
   */
  chatId: string;
};

/**
 * Describes the message analytics.ChatLeftEvent.
 * Use `create(ChatLeftEventSchema)` to create a new message.
 */
export const ChatLeftEventSchema: GenMessage<ChatLeftEvent> = /*@__PURE__*/
  messageDesc(file_messages, 9);

/**
 * / 页面切换事件
 *
 * @generated from message analytics.NavigationEvent
 */
export type NavigationEvent = Message<"analytics.NavigationEvent"> & {
  /**
   * / 当前页面
   *
   * @generated from field: string from = 1;
   */
  from: string;

  /**
   * / 目标页面
   *
   * @generated from field: string to = 2;
   */
  to: string;
};

/**
 * Describes the message analytics.NavigationEvent.
 * Use `create(NavigationEventSchema)` to create a new message.
 */
export const NavigationEventSchema: GenMessage<NavigationEvent> = /*@__PURE__*/
  messageDesc(file_messages, 10);

/**
 * / 事件上下文
 *
 * @generated from message analytics.EventContext
 */
export type EventContext = Message<"analytics.EventContext"> & {
  /**
   * / 客户端 ID
   *
   * @generated from field: string client_id = 1;
   */
  clientId: string;

  /**
   * / 应用版本
   *
   * @generated from field: string app_version = 2;
   */
  appVersion: string;

  /**
   * / 系统信息
   *
   * @generated from field: analytics.SystemInfo system = 3;
   */
  system?: SystemInfo;

  /**
   * / 用户 ID
   *
   * @generated from field: string user_id = 4;
   */
  userId: string;

  /**
   * / 客户端 IP
   *
   * @generated from field: string ip = 5;
   */
  ip: string;

  /**
   * / 用户代理
   *
   * @generated from field: string user_agent = 6;
   */
  userAgent: string;

  /**
   * / 地理位置
   *
   * @generated from field: analytics.GeoLocation geo = 7;
   */
  geo?: GeoLocation;

  /**
   * / 客户端时间戳
   *
   * @generated from field: int64 client_ts = 8;
   */
  clientTs: bigint;

  /**
   * / 服务器时间戳
   *
   * @generated from field: int64 server_ts = 9;
   */
  serverTs: bigint;
};

/**
 * Describes the message analytics.EventContext.
 * Use `create(EventContextSchema)` to create a new message.
 */
export const EventContextSchema: GenMessage<EventContext> = /*@__PURE__*/
  messageDesc(file_messages, 11);

/**
 * / 系统信息
 *
 * @generated from message analytics.SystemInfo
 */
export type SystemInfo = Message<"analytics.SystemInfo"> & {
  /**
   * / 操作系统
   *
   * @generated from field: string os = 1;
   */
  os: string;

  /**
   * / 系统架构
   *
   * @generated from field: string arch = 2;
   */
  arch: string;

  /**
   * / 用户语言
   *
   * @generated from field: string locale = 3;
   */
  locale: string;

  /**
   * / 用户时区
   *
   * @generated from field: string timezone = 4;
   */
  timezone: string;
};

/**
 * Describes the message analytics.SystemInfo.
 * Use `create(SystemInfoSchema)` to create a new message.
 */
export const SystemInfoSchema: GenMessage<SystemInfo> = /*@__PURE__*/
  messageDesc(file_messages, 12);

/**
 * @generated from message analytics.GeoLocation
 */
export type GeoLocation = Message<"analytics.GeoLocation"> & {
  /**
   * @generated from field: string country = 1;
   */
  country: string;

  /**
   * @generated from field: string region = 2;
   */
  region: string;

  /**
   * @generated from field: string city = 3;
   */
  city: string;
};

/**
 * Describes the message analytics.GeoLocation.
 * Use `create(GeoLocationSchema)` to create a new message.
 */
export const GeoLocationSchema: GenMessage<GeoLocation> = /*@__PURE__*/
  messageDesc(file_messages, 13);
