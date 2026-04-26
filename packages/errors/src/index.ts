/**
 * Common wire error codes across the Phenotype platform.
 */
export enum ErrorCode {
  // Generic
  INTERNAL_ERROR = "INTERNAL_ERROR",
  INVALID_ARGUMENT = "INVALID_ARGUMENT",
  NOT_FOUND = "NOT_FOUND",
  ALREADY_EXISTS = "ALREADY_EXISTS",
  PERMISSION_DENIED = "PERMISSION_DENIED",
  UNAUTHENTICATED = "UNAUTHENTICATED",
  RESOURCE_EXHAUSTED = "RESOURCE_EXHAUSTED",
  CANCELLED = "CANCELLED",
  UNAVAILABLE = "UNAVAILABLE",
  NOT_IMPLEMENTED = "NOT_IMPLEMENTED",
  TIMEOUT = "TIMEOUT",

  // Protocol/Bus specific
  VALIDATION_ERROR = "VALIDATION_ERROR",
  METHOD_NOT_SUPPORTED = "METHOD_NOT_SUPPORTED",
  MISSING_CORRELATION_ID = "MISSING_CORRELATION_ID",

  // Terminal/Lane/Session
  TERMINAL_NOT_FOUND = "TERMINAL_NOT_FOUND",
  LANE_NOT_FOUND = "LANE_NOT_FOUND",
  SESSION_NOT_FOUND = "SESSION_NOT_FOUND",
  SESSION_NOT_ATTACHED = "SESSION_NOT_ATTACHED",
  TERMINAL_BINDING_INVALID = "TERMINAL_BINDING_INVALID",
}

export const ERROR_CODES = [
  ErrorCode.INTERNAL_ERROR,
  ErrorCode.INVALID_ARGUMENT,
  ErrorCode.NOT_FOUND,
  ErrorCode.ALREADY_EXISTS,
  ErrorCode.PERMISSION_DENIED,
  ErrorCode.UNAUTHENTICATED,
  ErrorCode.RESOURCE_EXHAUSTED,
  ErrorCode.CANCELLED,
  ErrorCode.UNAVAILABLE,
  ErrorCode.NOT_IMPLEMENTED,
  ErrorCode.TIMEOUT,
  ErrorCode.VALIDATION_ERROR,
  ErrorCode.METHOD_NOT_SUPPORTED,
  ErrorCode.MISSING_CORRELATION_ID,
  ErrorCode.TERMINAL_NOT_FOUND,
  ErrorCode.LANE_NOT_FOUND,
  ErrorCode.SESSION_NOT_FOUND,
  ErrorCode.SESSION_NOT_ATTACHED,
  ErrorCode.TERMINAL_BINDING_INVALID,
] as const satisfies readonly ErrorCode[];

export interface PhenotypeErrorEnvelope {
  readonly code: ErrorCode;
  readonly message: string;
  readonly details?: Record<string, unknown>;
  readonly fatal?: boolean;
  readonly retryable?: boolean;
}

export type HeliosErrorDetails = PhenotypeErrorEnvelope;

export class HeliosAppError extends Error {
  readonly code: ErrorCode;
  readonly details?: Record<string, unknown>;
  readonly fatal: boolean;
  readonly retryable?: boolean;

  constructor(code: ErrorCode, message: string, options?: {
    details?: Record<string, unknown>;
    fatal?: boolean;
    retryable?: boolean;
  }) {
    super(message);
    this.name = "HeliosAppError";
    this.code = code;
    this.details = options?.details;
    this.fatal = options?.fatal ?? false;
    this.retryable = options?.retryable;
  }

  toJSON(): PhenotypeErrorEnvelope {
    return {
      code: this.code,
      message: this.message,
      details: this.details,
      fatal: this.fatal,
      retryable: this.retryable,
    };
  }
}
