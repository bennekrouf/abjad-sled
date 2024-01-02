# Run

decay_factor: 
- high : impact of older data diminishes quickly

````bash
APP_ENV=local cargo watch -c -w src -x run
APP_ENV=staging cargo watch -c -w src -x run
APP_ENV=production cargo watch -c -w src -x run
```

```
APP_ENV=local cargo run
APP_ENV=staging cargo run
APP_ENV=production cargo run
```

iOS

```
    npx react-native bundle --entry-file ./index.tsx --platform ios --bundle-output ios/main.jsbundle --assets-dest ios
```

Android
```
    npx react-native bundle --platform android --dev false --entry-file index.tsx --bundle-output android/app/src/main/assets/index.android.bundle --assets-dest android/app/src/main/res/
```
