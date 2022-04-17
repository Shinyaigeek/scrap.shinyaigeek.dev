import firebase from "firebase/app";
import "firebase/auth";
import { User } from "./context";

export interface AuthUserState {
  user: User | null;
  token: string | null;
}

export const initialAuthUserState: AuthUserState = {
  user: null,
  token: null,
};

const LOGIN = "LOGIN" as const;
const SUCCESS_LOGIN = "SUCCESS_LOGIN" as const;
const FAIL_LOGIN = "FAIL_LOGIN" as const;
const LOGOUT = "LOGOUT" as const;
const SIGNUP = "SIGNUP" as const;
const SUCCESS_SIGNUP = "SUCCESS_SIGNUP" as const;
const FAIL_SIGNUP = "FAIL_SIGNUP" as const;

const loginAction = () => {
  return { type: LOGIN };
};

const successLoginAction = (user: AuthUserState) => {
  return { type: SUCCESS_LOGIN, payload: user };
};

const failLoginAction = () => {
  return { type: FAIL_LOGIN };
};

const logoutAction = () => {
  return { type: LOGOUT };
};

const signupAction = () => {
  return { type: SIGNUP };
};

const successSignUpAction = (user: AuthUserState) => {
  return { type: SUCCESS_SIGNUP, payload: user };
};

const failSignUpAction = () => {
  return { type: FAIL_SIGNUP };
};

export const authUserActions = {
  loginAction,
  successLoginAction,
  failLoginAction,
  logoutAction,
  signupAction,
  successSignUpAction,
  failSignUpAction,
};

export type ActionType =
  | ReturnType<typeof loginAction>
  | ReturnType<typeof successLoginAction>
  | ReturnType<typeof failLoginAction>
  | ReturnType<typeof signupAction>
  | ReturnType<typeof successSignUpAction>
  | ReturnType<typeof failSignUpAction>
  | ReturnType<typeof logoutAction>;

export const authUserReducer = (
  state: AuthUserState,
  action: ActionType
): AuthUserState => {
  switch (action.type) {
    case LOGIN:
      return {
        ...state,
      };
    case SUCCESS_LOGIN:
      return {
        ...state,
        ...action.payload,
      };
    case FAIL_LOGIN:
      return {
        ...state,
      };
    case SIGNUP:
      return {
        ...state,
      };
    case SUCCESS_SIGNUP:
      return {
        ...state,
        ...action.payload,
      };
    case FAIL_SIGNUP:
      return {
        ...state,
      };
    case LOGOUT:
      return {
        ...state,
      };
    default:
      return state;
  }
};
