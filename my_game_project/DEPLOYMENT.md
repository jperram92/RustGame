# Deployment Guide

This guide explains how to deploy the Tic-Tac-Toe game to Render and GitHub Pages.

## Deploying the Server to Render

### Prerequisites

- A GitHub account
- A Render account (sign up at https://render.com/)

### Steps

1. **Fork or Push the Repository to GitHub**
   - Create a new repository on GitHub
   - Push your code to the repository

2. **Sign Up for Render**
   - Go to https://render.com/
   - Sign up for a free account
   - Connect your GitHub account

3. **Create a New Web Service**
   - In your Render dashboard, click "New" and select "Blueprint"
   - Select your GitHub repository
   - Render will detect the `render.yaml` file and set up the service
   - Click "Apply"

4. **Configure Environment Variables (Optional)**
   - In your service settings, you can add additional environment variables
   - For example, you might want to set `RUST_LOG=debug` for more detailed logging

5. **Deploy the Service**
   - Render will automatically build and deploy your service
   - This may take a few minutes for the first deployment

6. **Get Your Service URL**
   - Once deployed, Render will provide a URL for your service
   - It will look like `https://your-app-name.onrender.com`
   - Save this URL for the next step

## Deploying the Client to GitHub Pages

### Prerequisites

- A GitHub account

### Steps

1. **Create a New Repository for the Client**
   - Go to GitHub and create a new repository
   - Name it something like `tic-tac-toe-client`
   - Make it public

2. **Update the API URL**
   - Open `client-deploy/index.html`
   - Find the line with `const API_URL = 'https://your-app-name.onrender.com';`
   - Replace `your-app-name` with your actual Render app name

3. **Push the Client Code to GitHub**
   - Initialize a new Git repository in the `client-deploy` directory
   - Add the GitHub repository as a remote
   - Push the code to GitHub

   ```bash
   cd client-deploy
   git init
   git add .
   git commit -m "Initial commit"
   git remote add origin https://github.com/your-username/tic-tac-toe-client.git
   git push -u origin main
   ```

4. **Enable GitHub Pages**
   - Go to your repository on GitHub
   - Click on "Settings"
   - Scroll down to the "GitHub Pages" section
   - Select the "main" branch as the source
   - Click "Save"

5. **Access Your Deployed Client**
   - GitHub Pages will provide a URL for your client
   - It will look like `https://your-username.github.io/tic-tac-toe-client`
   - Your game is now accessible online!

## Testing the Deployment

1. Open your GitHub Pages URL in a web browser
2. Click "New Game" to create a new game
3. Make moves by clicking on the board
4. Try the AI opponent by selecting a difficulty and clicking "AI Move"

## Troubleshooting

### Server Issues

- **Build Failures**: Check the build logs in Render for errors
- **Runtime Errors**: Check the logs in Render for runtime errors
- **Connection Issues**: Make sure your client is using the correct API URL

### Client Issues

- **CORS Errors**: Make sure your server allows cross-origin requests
- **API Connection Errors**: Check the browser console for network errors
- **GitHub Pages Not Updating**: It may take a few minutes for changes to propagate

## Maintenance

- **Updating the Server**: Push changes to your GitHub repository, and Render will automatically rebuild and deploy
- **Updating the Client**: Push changes to your client repository, and GitHub Pages will automatically update
