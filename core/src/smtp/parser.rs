//! Parse the SMTP responses to get information about the email address.

use super::error::SmtpError;
use crate::EmailAddress;
use async_smtp::error::Error as AsyncSmtpError;

/// What a rejected `RCPT TO` tells us about the mailbox.
#[derive(Debug, PartialEq, Eq)]
pub enum RcptRejection {
	/// The reply says nothing reliable about the mailbox.
	Inconclusive,
	/// The mailbox doesn't exist.
	Invalid,
	/// The provider disabled the mailbox.
	Disabled,
	/// The mailbox exists but has no room for new mail.
	FullInbox,
	/// The mailbox exists and is throttling incoming mail.
	Deliverable,
}

/// Classify the error that async-smtp returned for `RCPT TO`, lowercased.
/// Its text starts with "permanent: " for 5xx replies and "transient: " for
/// 4xx replies.
pub fn classify_rcpt_rejection(e: &str, email: &EmailAddress) -> RcptRejection {
	// Greylisting and blocks on our side are often worded like recipient
	// rejections ("Recipient address rejected: Greylisted", "Sender address
	// rejected: User unknown"), so they must be ruled out first.
	if is_greylisted(e) || is_rejected_by_policy(e) {
		return RcptRejection::Inconclusive;
	}

	// A 4xx reply defers the decision, e.g. "Recipient address rejected:
	// unverified address: Address verification in progress", so only a 5xx
	// reply can show that a mailbox is gone or disabled.
	let permanent = e.starts_with("permanent: ");
	if permanent && is_disabled_account(e) {
		return RcptRejection::Disabled;
	}
	// Over quota is usually a 4xx reply (4.2.2), and it still shows that the
	// mailbox exists.
	if is_full_inbox(e) {
		return RcptRejection::FullInbox;
	}
	// 4.2.1 The user you are trying to contact is receiving mail at a rate that
	if e.contains("the user you are trying to contact is receiving mail at a rate that") {
		return RcptRejection::Deliverable;
	}
	// Microsoft 365 answers with a 4.4.4 deferral for tenants that have no
	// mailboxes at all, and retrying never changes it.
	let known_final_transient = e.contains("no mail-enabled subscriptions");
	if (permanent || known_final_transient) && is_invalid(e, email) {
		return RcptRejection::Invalid;
	}
	RcptRejection::Inconclusive
}

/// is_greylisted checks for SMTP responses meaning that the server
/// temporarily deferred the recipient because of greylisting. These responses
/// often also contain "recipient address rejected", so they must be checked
/// before `is_invalid`, otherwise a valid mailbox would be flagged as invalid.
pub fn is_greylisted(e: &str) -> bool {
	// 450 4.2.0 <EMAIL>: Recipient address rejected: Greylisted, see http://postgrey.schweikert.ch/help/creativeconcern.com.html
	// 450 4.7.1 <EMAIL>: Recipient address rejected: Greylisting in action, please come back later
	// 450 4.7.1 Greylisted for 38 seconds
	e.contains("greylist")
		|| e.contains("graylist")
		|| e.contains("grey-list")
		|| e.contains("gray-list")
		|| e.contains("postgrey")
}

/// is_invalid checks for SMTP responses meaning that the email is invalid,
/// i.e. that the mailbox doesn't exist. It only looks at the wording;
/// `classify_rcpt_rejection` also requires a permanent reply.
pub fn is_invalid(e: &str, email: &EmailAddress) -> bool {
	// 550 Address rejected
	// 550 5.1.1 : Recipient address rejected
	// 550 5.1.1 : Recipient address rejected: User unknown in virtual alias table
	// 550 5.1.1 <EMAIL: Recipient address rejected: User unknown in relay recipient table
	e.contains("address rejected")
	// 550 5.1.1 : Unrouteable address
	|| e.contains("unrouteable")
	// 550 5.1.1 : The email account that you tried to reach does not exist
	|| e.contains("does not exist")
	// 550 invalid address
	// 550 User not local or invalid address – Relay denied
	|| e.contains("invalid address")
	// 5.1.1 Invalid email address
	|| e.contains("invalid email address")
	// 550 Invalid recipient
	|| e.contains("invalid recipient")
	|| e.contains("may not exist")
	|| e.contains("recipient invalid")
	// 550 5.1.1 : Recipient rejected
	|| e.contains("recipient rejected")
	// permanent: 5.1.1 Unknown recipient address
	|| e.contains("unknown recipient address")
	// 554 Unknown Recipient (#5.1.1) (on @parkwayhonda.com)
	|| e.contains("unknown recipient") 
	|| e.contains("undeliverable")
	// 550 User unknown
	// 550 5.1.1 <EMAIL> User unknown
	// 550 recipient address rejected: user unknown in local recipient table
	|| e.contains("user unknown")
	// 550 Unknown user
	|| e.contains("unknown user")
	// 5.1.1 Recipient unknown <EMAIL>
	|| e.contains("recipient unknown")
	// 550 5.1.1 No such user - pp
	// 550 No such user here
	|| e.contains("no such user")
	// permanent: 5.1.1 MXIN501 mailbox <EMAIL> unknown (on @virginmedia.com)
	// `e` is lowercased, so the address has to be as well.
	|| e.contains(format!("mailbox {} unknown", email.to_string().to_lowercase()).as_str())
	// 550 5.1.1 : Mailbox not found
	// 550 Unknown address error ‘MAILBOX NOT FOUND’
	|| e.contains("mailbox not found")
	// 550 5.1.1 : Invalid mailbox
	|| e.contains("invalid mailbox")
	// 550 5.1.1 Sorry, no mailbox here by that name
	|| e.contains("no mailbox")
	// 5.2.0 No such mailbox
	|| e.contains("no such mailbox")
	// 550 Requested action not taken: mailbox unavailable
	|| e.contains("mailbox unavailable")
	// 5.5.0 Requested actions not taken as the mailbox is unavailable (on @etu.uca.fr)
	|| e.contains("mailbox is unavailable")
	// 550 5.1.1 Is not a valid mailbox
	|| e.contains("not a valid mailbox")
	// No such recipient here
	|| e.contains("no such recipient")
	// 554 delivery error: This user doesn’t have an account
	|| e.contains("have an account")
	// 554 delivery error: dd This user doesn't have a yahoo.com account (<EMAIL>) [0] - mta4466.mail.bf1.yahoo.com (on @yahoo.com, @aol.com)
	// "doesn" matches both the straight and the curly apostrophe.
	|| (e.contains("this user doesn") && e.contains(" account"))
	// permanent: Unknown local part <USER> in <USER@flabeg.com> (on @flabeg.com)
	|| e.contains("unknown local part")
	// 5.1.1 RCP-P1 Domain facebook.com no longer available https://www.facebook.com/postmaster/response_codes?ip=3.80.111.155#RCP-P1
	|| e.contains("no longer available")
	// permanent: RCPT (<EMAIL>) dosn't exist (on @hgy.ooo, @stigpods.com.cn)
	|| e.contains("dosn't exist") // sic! typo is intentional
	// 5.1.1 <EMAIL>: Email address could not be found, or was misspelled (G8) (on @biotech-calendar.com, @invoicefactoring.com)
	|| e.contains("could not be found") 
	// No such person at this address (on @aconsa.com.mx)
	|| e.contains("no such person")
	// Callout verification failed: 550 No Such User Here (on @medipro.co.uk)
	|| e.contains("no such user")
	// 5.1.1 <EMAIL> Address Error (on @lucidity.co.za)
	|| e.contains("address error")
	// E-mail address is not handled by this system (on @kaimayfair.co.uk)
	|| e.contains("address is not handled")
	// permanent: 5.1.1 recipient is not exist (on @sim.com)
	|| e.contains("recipient is not exist")
	// permanent: 5.1.1 <EMAIL>  Recipient not found. (on @4polymer.com)
	|| e.contains("recipient not found")
	// permanent: 5.7.1 Email doesn't exist. Please forward it or send it to contact@magency.f; 5.7.1 r - gcdp 38308e7fff4ca-30344241c74si13773751fa.513 - gsmtp"
	|| e.contains("email doesn't exist")
	// permanent: verify address failed, User not found ***@salemall.vn (on @salemall.vn)
	|| e.contains("verify address failed")
	// transient: unable to verify user (on @computan.net). Only the 5xx form
	// counts: `classify_rcpt_rejection` treats the 4xx one as a deferral.
	|| e.contains("unable to verify user")
	// permanent: 5.1.1 Utilisateur inconnu (on @sante.fr)
	|| e.contains("utilisateur inconnu")
	// transient: 4.4.4 Mail received as unauthenticated, incoming to a recipient domain configured in a hosted tenant which has no mail-enabled subscriptions. (on Microsoft 365 tenants without Exchange Online, e.g. @biked.com)
	|| e.contains("no mail-enabled subscriptions")
	// permanent: 5.1.1 (bad destination mailbox address). There is no such
	// catch-all for 5.7.1, which is a policy rejection, e.g. "5.7.1 Service
	// unavailable; Client host [IP] blocked using zen.spamhaus.org".
	|| e.contains("permanent: 5.1.1")
}

/// Check that the mailbox has a full inbox.
pub fn is_full_inbox(e: &str) -> bool {
	// Only storage of the mailbox counts. "452 4.3.1 Insufficient system
	// resources" means the server as a whole is short on disk or memory.
	(e.contains("insufficient") && (e.contains("mailbox") || e.contains("quota")))
	// https://answers.microsoft.com/en-us/outlook_com/forum/all/how-do-i-interpret-the-delivery-failure-message/2f1bf9c0-8b03-4f8f-aacc-5f6ba60a73f3
	|| e.contains("mailbox full")
	// https://answers.microsoft.com/en-us/outlook_com/forum/all/how-do-i-interpret-the-delivery-failure-message/2f1bf9c0-8b03-4f8f-aacc-5f6ba60a73f3
	|| e.contains("quota exceeded")
	|| e.contains("over quota")
	// 550 user has too many messages on the server
	|| e.contains("too many messages")
	// transient: 4.2.2 The recipient's inbox is out of storage space. Please direct the; 4.2.2 recipient to; 4.2.2  https://support.google.com/mail/?p=OverQuotaTemp 41be03b00d2f7-801d5c00011si4287631a12.311 - gsmtp (on @gmail.com)
	|| e.contains("out of storage space")
	// permanent: Requested mail action aborted: exceeded storage allocation (on @gmx.de)
	|| e.contains("exceeded storage allocation")
}

/// Check if the email account has been disabled or blocked by the email
/// provider.
pub fn is_disabled_account(e: &str) -> bool {
	// 554 The email account that you tried to reach is disabled. Learn more at https://support.google.com/mail/?p=DisabledUser"
	e.contains("disabled")
	// 554 delivery error: Sorry your message to <EMAIL> cannot be delivered. This account has been disabled or discontinued
 || e.contains("discontinued")
 //550 5.2.1 RACT MY.IP: Mailbox is inactive: <USER@hanmail.net><CRLF> (on hanmail.net)
 || e.contains("inactive")
}

/// Check if the error is an IO "incomplete" error.
pub fn is_err_io_errors(e: &SmtpError) -> bool {
	match e {
		SmtpError::AsyncSmtpError(AsyncSmtpError::Io(err)) => err.to_string() == "incomplete",
		_ => false,
	}
}

/// The lowercased text of an SMTP error reply, without the "permanent: " or
/// "transient: " prefix. `None` for errors that aren't replies.
fn reply_text(e: &SmtpError) -> Option<String> {
	match e {
		SmtpError::AsyncSmtpError(AsyncSmtpError::Transient(r) | AsyncSmtpError::Permanent(r)) => {
			// TODO We can use .to_string() after:
			// https://github.com/async-email/async-smtp/pull/53
			Some(r.message.join("; ").to_lowercase())
		}
		_ => None,
	}
}

/// Check whether the server refused us rather than the recipient: our IP is
/// on a blocklist or has no reverse DNS, or the server rejected our client
/// host, HELO name or MAIL FROM address. These replies say nothing about the
/// mailbox.
fn is_rejected_by_policy(e: &str) -> bool {
	// Enhanced security/policy status takes precedence over ambiguous wording.
	e.split_whitespace().any(|word| {
		let word = word.trim_matches(|c: char| !c.is_ascii_digit() && c != '.');
		let mut parts = word.split('.');
		matches!(parts.next(), Some("4" | "5"))
			&& parts.next() == Some("7")
			&& parts.next().is_some_and(|detail| {
				!detail.is_empty() && detail.len() <= 3 && detail.bytes().all(|c| c.is_ascii_digit())
			})
			&& parts.next().is_none()
	})
	|| is_ip_blocked(e)
	|| needs_rdns(e)
	// 550 5.1.0 <FROM_EMAIL>: Sender address rejected: User unknown in local recipient table
	|| e.contains("sender address rejected")
	// 554 5.7.1 <unknown[IP]>: Client host rejected: Access denied
	|| e.contains("client host rejected")
	// 504 5.5.2 <HELO_NAME>: Helo command rejected: need fully-qualified hostname
	|| e.contains("helo command rejected")
}

/// Check if the IP is blacklisted.
pub fn is_err_ip_blacklisted(e: &SmtpError) -> bool {
	reply_text(e).is_some_and(|e| {
		is_ip_blocked(&e)
		// 5.7.1 Email cannot be delivered. Reason: Email detected as Spam by spam filters.
		|| e.contains("spam")
		// 5.7.1 <unknown[23.129.64.100]>: Client host rejected: Access denied
		// 4.7.1 <EMAIL>: Relay access denied
		|| e.contains("access denied")
	})
}

/// The patterns of `is_err_ip_blacklisted` that are specific enough to
/// override a recipient verdict. "access denied" isn't: Microsoft 365 rejects
/// unknown recipients with "550 5.4.1 Recipient address rejected: Access
/// denied". Nor is a bare "spam".
fn is_ip_blocked(e: &str) -> bool {
	// Permanent errors

	// 5.7.1 IP address blacklisted by recipient
	// 5.7.1 Service unavailable; Client host [147.75.45.223] is blacklisted. Visit https://www.sophos.com/en-us/threat-center/ip-lookup.aspx?ip=147.75.45.223 to request delisting
	// 5.3.0 <EMAIL>... Mail from 147.75.45.223 rejected by Abusix blacklist (on @helsinki.fi)
	e.contains("blacklist")
	// Rejected because 23.129.64.213 is in a black list at b.barracudacentral.org
	|| e.contains("black list")
	// 5.7.1 Recipient not authorized, your IP has been found on a block list
	// gmx.net (mxgmx117) Nemesis ESMTP Service not available; No SMTP service; IP address is block listed.; For explanation visit https://www.gmx.net/mail/senderguidelines?c=bl (on @gmx.net, @web.de)
	|| e.contains("block list")
	// host 23.129.64.216 is listed at combined.mail.abusix.zone (127.0.0.12,
	|| e.contains("abusix")
	// 5.7.1 Relaying denied. IP name possibly forged [45.154.35.252]
	// 5.7.1 Relaying denied: You must check for new mail before sending mail. [23.129.64.216]
	|| e.contains("relaying denied")
	// sorry, mail from your location [5.79.109.48] is administratively denied (#5.7.1)
	|| e.contains("administratively denied")
	// 5.7.606 Access denied, banned sending IP [23.129.64.216]
	|| e.contains("banned")
	// Blocked - see https://ipcheck.proofpoint.com/?ip=23.129.64.192
	// 5.7.1 Mail from 23.129.64.183 has been blocked by Trend Micro Email Reputation Service.
	// 5.7.1 Service unavailable, Client host [23.129.64.184] blocked using Spamhaus.
	|| e.contains("blocked")
	// Connection rejected by policy [7.3] 38206, please visit https://support.symantec.com/en_US/article.TECH246726.html for more details about this error message.
	|| e.contains("connection rejected")
	// csi.mimecast.org Poor Reputation Sender. - https://community.mimecast.com/docs/DOC-1369#550 [6ATVl4DjOvSA6XNsWGoUFw.us31]
	// Your access to this mail system has been rejected due to the sending MTA\'s poor reputation. If you believe that this failure is in error, please contact the intended recipient via alternate means.
	|| e.contains("poor reputation")
	// JunkMail rejected - (gmail.com) [193.218.118.140]:46615 is in an RBL: http://www.barracudanetworks.com/reputation/?pr=1&ip=193.218.118.140
	|| e.contains("junkmail")
	// mailfi01.lmco.com ESMTP 550 5.7.0  Mail from 18.234.87.196 refused by Proofpoint Reputation Services.  SENDER please see and take action: https://support.proofpoint.com/dnsbl-lookup.cgi?18.234.87.196" (on @lmco.com)
	|| e.contains("refused by proofpoint")
	// resimta-h1p-037598.sys.comcast.net resimta-h1p-037598.sys.comcast.net 5.135.185.166 found on one or more DNSBLs, see http://postmaster.comcast.net/smtp-error-codes.php#BL000001 (on @comcast.net)
	|| e.contains("dnsbl")
	// smtp-fw-9107.amazon.com; SBRS score too low: http://www.senderbase.org/ (on @amazon.com)
	|| e.contains("sbrs score too low")
	// Unable to add <EMAIL> because host 23.129.64.184 is listed on zen.spamhaus.org
	// https://www.spamhaus.org/sbl/query/SBLCSShttps://www.spamhaus.org/query/ip/3.238.201.74 (on @knollridges.com.ph)
	|| e.contains("spamhaus")

	// Transient errors ("blocked" above also covers "Blocked - see
	// https://www.spamcop.net/bl.shtml?23.129.64.211")

	// relay not permitted!
	|| e.contains("relay not permitted")
	// 23.129.64.216 is not yet authorized to deliver mail from
	|| e.contains("not yet authorized")
}

/// Check if the IP needs a reverse DNS.
pub fn is_err_needs_rdns(e: &SmtpError) -> bool {
	reply_text(e).is_some_and(|e| needs_rdns(&e))
}

fn needs_rdns(e: &str) -> bool {
	// 4.7.25 Client host rejected: cannot find your hostname, [147.75.45.223]
	// 4.7.1 Client host rejected: cannot find your reverse hostname, [147.75.45.223]
	// 5.7.1 Client host rejected: cannot find your reverse hostname, [23.129.64.184]
	e.contains("cannot find your reverse hostname")
	// You dont seem to have a reverse dns entry. Come back later. You are greylisted for 20 minutes. See http://www.fsf.org/about/systems/greylisting
	|| e.contains("reverse dns entry")
}

#[cfg(test)]
mod tests {

	use super::{classify_rcpt_rejection, is_err_ip_blacklisted, RcptRejection};
	use crate::EmailAddress;
	use crate::SmtpError::AsyncSmtpError;
	use async_smtp::{
		error::Error,
		response::{Category, Code, Detail, Response, Severity},
	};
	use std::str::FromStr;

	/// Replies as `check_email_deliverability` sees them: async-smtp's error
	/// string ("permanent: " for 5xx, "transient: " for 4xx), lowercased.
	#[test]
	fn test_classify_rcpt_rejection() {
		use RcptRejection::*;
		// Mixed case on purpose: servers echo the address back in lowercase.
		let email = EmailAddress::from_str("Foo@Bar.baz").unwrap();

		let cases = [
			// The mailbox doesn't exist.
			(
				"permanent: 5.1.1 <foo@bar.baz>: recipient address rejected: user unknown in virtual mailbox table",
				Invalid,
			),
			("permanent: rcpt (***@stigpods.com.cn) dosn't exist", Invalid),
			// On @virginmedia.com, minus the enhanced status code, which
			// servers that don't support it leave out.
			("permanent: mxin501 mailbox foo@bar.baz unknown", Invalid),
			// On @yahoo.com and @aol.com.
			(
				"permanent: delivery error: dd this user doesn't have a yahoo.com account (foo@bar.baz) [0] - mta4466.mail.bf1.yahoo.com",
				Invalid,
			),
			// Microsoft 365 directory-based edge blocking. "Access denied"
			// here is about the recipient, not about us.
			(
				"permanent: 5.4.1 recipient address rejected: access denied. as(201806281) [bn8nam12ft036.eop-nam12.prod.protection.outlook.com]",
				Invalid,
			),
			// Policy status cannot establish mailbox absence, even with misleading text.
			(
				"permanent: 5.7.1 email doesn't exist. please forward it or send it to contact@magency.fr",
				Inconclusive,
			),
			(
				"permanent: 5.7.1 recipient address rejected: authentication required",
				Inconclusive,
			),
			(
				"permanent: recipient address rejected (#5.7.606)",
				Inconclusive,
			),
			// A Microsoft 365 tenant without mailboxes defers, but retrying
			// never helps. https://github.com/reacherhq/check-if-email-exists/issues/1666
			(
				"transient: 4.4.4 mail received as unauthenticated, incoming to a recipient domain configured in a hosted tenant which has no mail-enabled subscriptions. attr5 [sj5pepf000001cc.namprd05.prod.outlook.com 2026-10-13t12:16:22.797z 08df2669f6c8e25d]",
				Invalid,
			),
			// Blocks on our IP, reverse DNS, HELO name or sender address say
			// nothing about the mailbox, even when worded like a recipient
			// rejection.
			(
				"permanent: 5.7.1 service unavailable; client host [203.0.113.5] blocked using zen.spamhaus.org; https://www.spamhaus.org/query/ip/203.0.113.5",
				Inconclusive,
			),
			(
				"permanent: 5.7.1 [203.0.113.5] the ip you're using to send mail is not authorized to send; 5.7.1 email directly to our servers. please use the smtp relay at your; 5.7.1 service provider instead. https://support.google.com/mail/?p=notauthorizederror - gsmtp",
				Inconclusive,
			),
			(
				"permanent: 5.1.0 <me@example.org>: sender address rejected: user unknown in local recipient table",
				Inconclusive,
			),
			(
				"permanent: 5.7.1 <mta.voipdir.net[]>: client host rejected: access denied",
				Inconclusive,
			),
			(
				"permanent: 5.5.2 <localhost>: helo command rejected: need fully-qualified hostname",
				Inconclusive,
			),
			(
				"transient: 4.7.1 client host rejected: cannot find your reverse hostname, [203.0.113.5]",
				Inconclusive,
			),
			// A 4xx deferral doesn't prove the mailbox is missing.
			(
				"transient: 4.1.1 <foo@bar.baz>: recipient address rejected: unverified address: address verification in progress",
				Inconclusive,
			),
			// https://github.com/reacherhq/check-if-email-exists/issues/1658
			(
				"transient: 4.2.0 <foo@bar.baz>: recipient address rejected: greylisted, see http://postgrey.schweikert.ch/help/creativeconcern.com.html",
				Inconclusive,
			),
			(
				"transient: 4.7.1 <foo@bar.baz>: recipient address rejected: greylisting in action, please come back later",
				Inconclusive,
			),
			// The whole server is short on resources, not this mailbox.
			("transient: 4.3.1 insufficient system resources", Inconclusive),
			// https://github.com/reacherhq/check-if-email-exists/issues/1464
			(
				"permanent: requested mail action aborted: exceeded storage allocation",
				FullInbox,
			),
			("permanent: 5.2.2 quota exceeded", FullInbox),
			// Over quota is a deferral, but it does tell us the mailbox exists.
			(
				"transient: 4.2.2 the recipient's inbox is out of storage space. please direct the; 4.2.2 recipient to; 4.2.2  https://support.google.com/mail/?p=overquotatemp 41be03b00d2f7-801d5c00011si4287631a12.311 - gsmtp",
				FullInbox,
			),
			(
				"permanent: 5.2.1 the email account that you tried to reach is disabled. learn more at; 5.2.1  https://support.google.com/mail/?p=disableduser - gsmtp",
				Disabled,
			),
			(
				"transient: 4.2.1 the user you are trying to contact is receiving mail at a rate that; 4.2.1 prevents additional messages from being delivered. - gsmtp",
				Deliverable,
			),
		];

		let misclassified: Vec<String> = cases
			.iter()
			.map(|(reply, expected)| (reply, expected, classify_rcpt_rejection(reply, &email)))
			.filter(|(_, expected, actual)| actual != *expected)
			.map(|(reply, expected, actual)| {
				format!("expected {:?}, got {:?}: {}", expected, actual, reply)
			})
			.collect();
		assert!(misclassified.is_empty(), "{:#?}", misclassified);
	}

	#[test]
	fn test_is_err_ip_blacklisted() {
		let err = Error::Permanent(Response::new(
			Code::new(
				Severity::PermanentNegativeCompletion,
				Category::Information,
				Detail::Zero,
			),
			vec![
				"gmx.net (mxgmx117) Nemesis ESMTP Service not available".to_string(),
				"No SMTP service".to_string(),
				"IP address is block listed.".to_string(),
				"For explanation visit https://www.gmx.net/mail/senderguidelines?c=bl".to_string(),
			],
		));

		assert!(is_err_ip_blacklisted(&AsyncSmtpError(err)))
	}
}
