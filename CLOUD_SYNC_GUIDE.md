# Google Cloud Setup Guide for My Home

To enable Cloud Sync and Household Sharing, you need to set up a Google Cloud Project and obtain a `client_secret.json` file.

## 1. Create a Google Cloud Project
1. Go to the [Google Cloud Console](https://console.cloud.google.com/).
2. Click on the project dropdown and select **New Project**.
3. Name it `My Home App` and click **Create**.

## 2. Enable Google Drive API
1. In the sidebar, go to **APIs & Services > Library**.
2. Search for `Google Drive API`.
3. Click on it and click **Enable**.

## 3. Configure OAuth Consent Screen
1. Go to **APIs & Services > OAuth consent screen**.
2. Choose **External** and click **Create**.
3. Fill in the required fields (App name: `My Home`, User support email, Developer contact info).
4. In **Scopes**, click **Add or Remove Scopes**.
5. Manually add this scope: `https://www.googleapis.com/auth/drive.file` (This grants access only to files created by the app).
6. Add your own email to **Test users**.

## 4. Create OAuth 2.0 Credentials
1. Go to **APIs & Services > Credentials**.
2. Click **Create Credentials > OAuth client ID**.
3. Select **Desktop app** as the Application type.
4. Name it `My Home Desktop`.
5. Click **Create**.
6. Download the JSON file, rename it to `client_secret.json`, and place it in the root directory of the `myHome` project.

## 5. First Time Linking
1. Run the app: `just run`.
2. Go to **Settings > Cloud Account**.
3. Click **Link Google Account**.
4. Your browser will open a Google login page. Sign in and grant permission.
5. The app will store an encrypted token in the `.tokens/` folder for future syncs.
