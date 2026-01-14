 # Email Automation Guide for Quittance App

## Current Implementation: Simple Email (✅ Already Added)

A "📧 Envoyer par email" button that opens your default email client with pre-filled content. 

**Pros:**
- No backend needed
- Works on GitHub Pages
- Uses your regular email account

**Cons:**
- Requires manual sending
- Cannot attach PDF automatically
- Depends on user having email client configured

---

## Advanced Options for Full Automation

### Option 1: EmailJS (Recommended for Static Sites)

**EmailJS** allows sending emails from client-side JavaScript without a backend.

#### Setup:
1. Create account at [emailjs.com](https://www.emailjs.com/) (free tier: 200 emails/month)
2. Install package:
   ```bash
   npm install @emailjs/browser
   ```

3. Add to `ReceiptPreview.vue`:
   ```typescript
   import emailjs from '@emailjs/browser'

   async function sendByEmailJS() {
     try {
       // Generate PDF as base64
       const pdfBase64 = doc.output('datauristring').split(',')[1]
       
       await emailjs.send(
         'YOUR_SERVICE_ID',
         'YOUR_TEMPLATE_ID',
         {
           to_email: 'tenant@email.com',
           to_name: props.data.tenant.name,
           from_name: props.data.landlord.name,
           subject: `Quittance - ${periodLabel.value}`,
           message: 'Votre quittance de loyer',
           pdf_attachment: pdfBase64,
           pdf_name: `quittance_${periodLabel.value}.pdf`
         },
         'YOUR_PUBLIC_KEY'
       )
       
       alert('Email envoyé avec succès!')
     } catch (error) {
       alert('Erreur lors de l\'envoi')
     }
   }
   ```

**Pros:**
- No backend needed
- Can attach PDFs
- Easy to set up

**Cons:**
- Exposes API key in client (use restrictions)
- Limited free tier

---

### Option 2: Azure Functions (Best for Production)

Create a serverless function to send emails via Azure Communication Services or SendGrid.

#### Architecture:
```
Frontend (GitHub Pages) → Azure Function → Email Service → Recipient
```

#### Setup:
1. Create Azure Function App
2. Install dependencies:
   ```bash
   npm install @azure/communication-email
   ```

3. Function code (`SendReceipt/index.ts`):
   ```typescript
   import { EmailClient } from "@azure/communication-email"
   
   export default async function (context, req) {
     const client = new EmailClient(process.env.COMMUNICATION_CONNECTION_STRING)
     
     const message = {
       senderAddress: "noreply@yourdomain.com",
       content: {
         subject: req.body.subject,
         plainText: req.body.message,
       },
       recipients: {
         to: [{ address: req.body.toEmail }],
       },
       attachments: [{
         name: req.body.pdfName,
         contentType: "application/pdf",
         contentInBase64: req.body.pdfBase64
       }]
     }
     
     await client.beginSend(message)
     context.res = { status: 200, body: "Email sent" }
   }
   ```

4. Update `vite.config.ts` to proxy API calls:
   ```typescript
   export default defineConfig({
     server: {
       proxy: {
         '/api': 'https://your-function-app.azurewebsites.net'
       }
     }
   })
   ```

5. Call from frontend:
   ```typescript
   async function sendByEmail() {
     const pdfBase64 = doc.output('datauristring').split(',')[1]
     
     await fetch('/api/SendReceipt', {
       method: 'POST',
       headers: { 'Content-Type': 'application/json' },
       body: JSON.stringify({
         toEmail: 'tenant@email.com',
         subject: `Quittance - ${periodLabel.value}`,
         message: 'Votre quittance...',
         pdfBase64,
         pdfName: `quittance_${periodLabel.value}.pdf`
       })
     })
   }
   ```

**Pros:**
- Secure (credentials server-side)
- Scalable
- Professional solution
- Can add scheduling, database storage, etc.

**Cons:**
- Requires Azure account
- More complex setup
- Costs (but cheap with consumption plan)

---

### Option 3: Scheduled Automation (Monthly Sending)

For automatic monthly sending, combine Azure Functions with:

1. **Azure Logic Apps** - Visual workflow designer
2. **Timer Trigger** - Runs on 1st of each month
3. **Database** - Store tenant emails and rent details

**Workflow:**
```
Timer (1st of month) → Logic App → 
  For each tenant → 
    Generate PDF → 
    Send Email via Azure Communication Services
```

This allows completely hands-free monthly quittance sending!

---

### Option 4: Simple Backend (Node.js/Express)

If you prefer self-hosting:

1. Create simple Express server:
   ```typescript
   import express from 'express'
   import nodemailer from 'nodemailer'
   
   app.post('/api/send-receipt', async (req, res) => {
     const transporter = nodemailer.createTransport({
       service: 'gmail',
       auth: {
         user: process.env.EMAIL_USER,
         pass: process.env.EMAIL_PASSWORD
       }
     })
     
     await transporter.sendMail({
       from: process.env.EMAIL_USER,
       to: req.body.toEmail,
       subject: req.body.subject,
       html: req.body.message,
       attachments: [{
         filename: req.body.pdfName,
         content: Buffer.from(req.body.pdfBase64, 'base64')
       }]
     })
     
     res.json({ success: true })
   })
   ```

2. Deploy to any Node.js hosting (Railway, Render, Heroku, etc.)

---

## Recommended Approach

**For immediate use:** 
- Use the current mailto: button (already implemented)

**For better user experience:**
- Add EmailJS (30 min setup, works with GitHub Pages)

**For production/business:**
- Azure Functions + Azure Communication Services (most robust)

**For scheduled automation:**
- Azure Logic Apps + Storage (fully automated monthly sending)

---

## Cost Comparison

| Service | Free Tier | Paid |
|---------|-----------|------|
| mailto: | ✅ Free | - |
| EmailJS | 200/month | $7/month (1000 emails) |
| Azure Functions | 1M executions | Pay per use (~$0.20/10k) |
| Azure Communication | 500 emails/month | $0.00025/email |
| SendGrid | 100/day | $15/month (40k emails) |

---

## Next Steps

1. **Start simple**: Current mailto: button works for manual sending
2. **Try EmailJS**: If you want PDF attachments without backend (5-10 tenants)
3. **Go serverless**: For professional use with many tenants (>20)
4. **Add scheduling**: When you want fully automated monthly sending

Would you like help implementing any of these options?
