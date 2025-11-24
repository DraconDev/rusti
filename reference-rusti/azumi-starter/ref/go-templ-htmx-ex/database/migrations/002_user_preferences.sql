-- Add timezone and email_billing to user_preferences
ALTER TABLE user_preferences 
ADD COLUMN IF NOT EXISTS timezone VARCHAR(50) DEFAULT 'UTC',
ADD COLUMN IF NOT EXISTS email_billing BOOLEAN DEFAULT true;
