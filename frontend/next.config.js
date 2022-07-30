require("dotenv").config();

const MiniCssExtractPlugin = require("mini-css-extract-plugin");
// function withCustomWebpackConfig(pluginOptions = {}) {
//   /**
//    * @param {any} nextConfig Custom config for Next.js
//    */
//   return (nextConfig = {}) => {
//     return {
//       webpack(config, options) {
//         const { dev, isServer } = options;

//         config.module.rules.push({
//           test: /\.css$/i,
//           sideEffects: true,
//           use: [MiniCssExtractPlugin.loader, "css-loader"],
//         });

//         const plugins = [];

//         plugins.push(
//           new MiniCssExtractPlugin({
//             filename: "static/css/[contenthash].css",
//             chunkFilename: "static/css/[contenthash].css",
//             ignoreOrder: true,
//           })
//         );

//         config.plugins.push(...plugins);

//         if (typeof nextConfig.webpack === "function") {
//           return nextConfig.webpack(config, options);
//         }

//         return config;
//       },
//       env: {
//         FIREBASE_API_KEY: process.env.FIREBASE_API_KEY,
//         FIREBASE_AUTHDOMAIN: process.env.FIREBASE_AUTHDOMAIN,
//       },
//     };
//   };
// }

module.exports = /** withCustomWebpackConfig() */ {
  env: {
    FIREBASE_API_KEY: process.env.FIREBASE_API_KEY,
    FIREBASE_AUTHDOMAIN: process.env.FIREBASE_AUTHDOMAIN,
  },
};
