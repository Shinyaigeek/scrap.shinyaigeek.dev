import admin from "firebase-admin";
import _serviceAccount from "./scrap-shinyaigeek-dev-firebase-adminsdk-9geul-3b428d381c.json";

const serviceAccount: admin.ServiceAccount = {
    ..._serviceAccount,
    privateKey: _serviceAccount.private_key.replace(
      /\\n/g,
      "\n"
    ),
  };

/**
 * @description Firebase Admin SDKを扱うためのオブジェクト
 * @note バックエンドのみで使用可能
 */
export const firebaseAdmin =
  admin.apps[0] ||
  admin.initializeApp({
    credential: admin.credential.cert(serviceAccount as any),
  });
