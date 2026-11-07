import type { CredencialesPos } from '../../../types/pos';
export function AutorizacionPos({
  necesaria,
  valor,
  onChange,
}: {
  necesaria: boolean;
  valor: CredencialesPos;
  onChange: (c: CredencialesPos) => void;
}) {
  if (!necesaria) return null;
  return (
    <fieldset className="pos-auth">
      <legend>Autorización administrativa</legend>
      <p>El cajero y su caja conservarán la titularidad de esta operación.</p>
      <div className="pos-fields">
        <label>
          Usuario administrador
          <input
            autoComplete="off"
            value={valor.username}
            onChange={(e) => onChange({ ...valor, username: e.target.value })}
          />
        </label>
        <label>
          Contraseña
          <input
            type="password"
            autoComplete="new-password"
            value={valor.password}
            onChange={(e) => onChange({ ...valor, password: e.target.value })}
          />
        </label>
      </div>
    </fieldset>
  );
}
