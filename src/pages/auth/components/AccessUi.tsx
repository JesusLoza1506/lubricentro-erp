import React, { useEffect, useRef } from 'react';
import { AlertTriangle, CheckCircle2, Copy, ShieldAlert } from 'lucide-react';

export function AccessHeading({ label, title, description, focus = false }: {
  label: string;
  title: string;
  description: string;
  focus?: boolean;
}) {
  const heading = useRef<HTMLHeadingElement>(null);
  useEffect(() => {
    if (focus) heading.current?.focus({ preventScroll: true });
  }, [focus]);
  return (
    <header className="gloria-access-header">
      <p className="gloria-access-kicker">{label}</p>
      <h2 ref={heading} tabIndex={focus ? -1 : undefined}>{title}</h2>
      <p>{description}</p>
    </header>
  );
}

export function AccessNotice({ tone, children }: {
  tone: 'warning' | 'error' | 'success';
  children: React.ReactNode;
}) {
  const Icon = tone === 'error' ? ShieldAlert : tone === 'success' ? CheckCircle2 : AlertTriangle;
  return (
    <div className={`gt-notice gt-notice--${tone}`} role={tone === 'error' ? 'alert' : 'status'}>
      <Icon size={16} aria-hidden="true" />
      <span>{children}</span>
    </div>
  );
}

export function AccessField({ label, inputRef, icon, action, hint, ...input }: React.InputHTMLAttributes<HTMLInputElement> & {
  id: string;
  label: string;
  inputRef?: React.Ref<HTMLInputElement>;
  icon?: React.ReactNode;
  action?: React.ReactNode;
  hint?: string;
}) {
  const hintId = hint ? `${input.id}-hint` : undefined;
  return (
    <div className="gt-field">
      <label htmlFor={input.id}>{label}</label>
      <div className="gt-field-wrap">
        {icon && <span className="gt-field-icon" aria-hidden="true">{icon}</span>}
        <input {...input} ref={inputRef} aria-describedby={hintId}
          className={`gt-input${icon ? ' gt-input--leading' : ''}${action ? ' gt-input--trailing' : ''}`} />
        {action}
      </div>
      {hint && <p className="gt-field-hint" id={hintId}>{hint}</p>}
    </div>
  );
}

export function AccessSubmit({ busy, busyLabel, children, disabled = busy }: {
  busy: boolean;
  busyLabel: string;
  children: React.ReactNode;
  disabled?: boolean;
}) {
  return (
    <button type="submit" className="gt-button gt-button--primary" disabled={disabled} aria-busy={busy}>
      {busy && <span className="gt-spinner" aria-hidden="true" />}
      {busy ? busyLabel : children}
    </button>
  );
}

export function RecoveryCode({ code, copied, onCopy, onContinue, confirmation }: {
  code: string;
  copied: boolean;
  onCopy: (code: string) => void;
  onContinue: () => void;
  confirmation?: { checked: boolean; onChange: (value: boolean) => void };
}) {
  return (
    <div className="gloria-code">
      <code className="gloria-code-value" tabIndex={0} aria-label="Código de recuperación">{code}</code>
      <button type="button" className="gt-button" data-copied={copied} onClick={() => onCopy(code)}>
        {copied ? <CheckCircle2 size={16} aria-hidden="true" /> : <Copy size={16} aria-hidden="true" />}
        <span role="status">{copied ? 'Copiado' : 'Copiar código'}</span>
      </button>
      {confirmation && (
        <label className="gloria-code-confirm">
          <input type="checkbox" checked={confirmation.checked} onChange={(e) => confirmation.onChange(e.target.checked)} />
          He guardado el código en un lugar seguro.
        </label>
      )}
      <button type="button" className="gt-button gt-button--primary"
        disabled={confirmation ? !confirmation.checked : false} onClick={onContinue}>
        Ir a iniciar sesión
      </button>
    </div>
  );
}
