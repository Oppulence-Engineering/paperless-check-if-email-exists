-- Drain all external verification work before removing paid-attempt recovery state.
DROP TABLE verification_provider_attempts;
