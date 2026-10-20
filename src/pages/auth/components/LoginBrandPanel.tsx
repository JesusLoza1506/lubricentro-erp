import React from 'react';
import logo from '../../../assets/logo-gloria-login.png';
import { loginStyles as s } from '../../../styles/loginStyles';

interface LoginBrandPanelProps {
  appVersion: string;
}

export const LoginBrandPanel: React.FC<LoginBrandPanelProps> = ({ appVersion }) => (
  <section className="gloria-brand" style={s.leftPanel} aria-label="Lubricentro Gloria">
    <header className="gloria-brand-header">
      <img className="gloria-brand-logo" src={logo} alt="Lubricentro Gloria" />
      <span className="gloria-brand-caption">Sistema de gestión</span>
    </header>

    <div className="gloria-brand-copy">
      <span className="gloria-brand-rule" aria-hidden="true" />
      <h1>La gestión de tu taller,<br /><span>en un solo lugar.</span></h1>
      <p>Administra vehículos, servicios, ventas e inventario.</p>
    </div>

    <footer className="gloria-brand-footer">
      <span>© 2026 Lubricentro Gloria S.R.L.</span>
      <span>Versión {appVersion}</span>
    </footer>
  </section>
);
