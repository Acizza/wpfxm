export interface DisplayError {
  context: string;
  message: string;
}

export type ErrorClosure = (error: DisplayError | undefined) => void;

export default {};
