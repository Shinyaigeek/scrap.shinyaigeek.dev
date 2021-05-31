require("dotenv").config();

module.exports = {
  env: {
    FIREBASE_API_KEY: process.env.FIREBASE_API_KEY,
    FIREBASE_AUTHDOMAIN: process.env.FIREBASE_AUTHDOMAIN
  },
};
