import { useTranslation } from "react-i18next";
import { useQuery } from "@tanstack/react-query";
import { tauriInvoke } from "@/shared/api/tauri-client";
import { useThemeStore } from "@/shared/hooks/use-theme";
import { useLocaleStore } from "@/shared/stores/use-locale-store";
import { supportedLocales, localeLabels } from "@/shared/config/i18n";
import { Sun, Moon } from "lucide-react";
import type { Config } from "@/shared/types/config";

export function SettingsPage() {
  const { t } = useTranslation();
  const { theme, toggleTheme } = useThemeStore();
  const { locale, setLocale } = useLocaleStore();

  const { data: config } = useQuery({
    queryKey: ["config"],
    queryFn: () => tauriInvoke<Config>("get_config"),
  });

  return (
    <div className="space-y-6">
      <h1 className="text-text text-2xl font-bold">{t("settings.title")}</h1>

      {/* Apparence */}
      <div className="bg-mantle rounded-xl border p-4">
        <h2 className="text-text mb-4 font-semibold">{t("settings.appearance")}</h2>
        <div className="flex items-center justify-between">
          <span className="text-subtext-0 text-sm">{t("settings.theme")}</span>
          <button
            onClick={toggleTheme}
            className="bg-surface-0 hover:bg-surface-1 flex items-center gap-2 rounded-lg px-3 py-2 text-sm transition-colors"
          >
            {theme === "dark" ? <Moon className="h-4 w-4" /> : <Sun className="h-4 w-4" />}
            {theme === "dark" ? t("settings.dark") : t("settings.light")}
          </button>
        </div>
        <div className="mt-4 flex items-center justify-between">
          <span className="text-subtext-0 text-sm">{t("settings.language")}</span>
          <div className="flex gap-1">
            {supportedLocales.map((loc) => (
              <button
                key={loc}
                onClick={() => setLocale(loc)}
                className={`rounded-lg px-3 py-1.5 text-xs transition-colors ${
                  locale === loc ? "bg-blue text-crust" : "bg-surface-0 text-text hover:bg-surface-1"
                }`}
              >
                {localeLabels[loc]}
              </button>
            ))}
          </div>
        </div>
      </div>

      {/* LLM Config */}
      {config && (
        <div className="bg-mantle rounded-xl border p-4">
          <h2 className="text-text mb-4 font-semibold">{t("settings.llm")}</h2>
          <div className="grid grid-cols-2 gap-4 text-sm">
            <div>
              <span className="text-subtext-0 text-xs">{t("settings.provider")}</span>
              <p className="text-text">{config.llm.provider}</p>
            </div>
            <div>
              <span className="text-subtext-0 text-xs">{t("settings.model")}</span>
              <p className="text-text">{config.llm.model_name}</p>
            </div>
            <div>
              <span className="text-subtext-0 text-xs">{t("settings.url")}</span>
              <p className="text-text">{config.llm.api_base_url}</p>
            </div>
            <div>
              <span className="text-subtext-0 text-xs">{t("settings.temperature")}</span>
              <p className="text-text">{config.llm.temperature}</p>
            </div>
            <div>
              <span className="text-subtext-0 text-xs">{t("settings.maxTokens")}</span>
              <p className="text-text">{config.llm.max_tokens}</p>
            </div>
            <div>
              <span className="text-subtext-0 text-xs">{t("settings.contextSize")}</span>
              <p className="text-text">{config.llm.context_size}</p>
            </div>
          </div>
        </div>
      )}

      {/* Pipeline Config */}
      {config && (
        <div className="bg-mantle rounded-xl border p-4">
          <h2 className="text-text mb-4 font-semibold">{t("settings.pipeline")}</h2>
          <div className="grid grid-cols-2 gap-4 text-sm">
            <div>
              <span className="text-subtext-0 text-xs">{t("settings.defaultLanguage")}</span>
              <p className="text-text">{config.pipeline.default_language}</p>
            </div>
            <div>
              <span className="text-subtext-0 text-xs">{t("settings.maxRetries")}</span>
              <p className="text-text">{config.pipeline.max_retries}</p>
            </div>
            <div>
              <span className="text-subtext-0 text-xs">{t("settings.compliance")}</span>
              <p className="text-text">{config.compliance.profile}</p>
            </div>
            <div>
              <span className="text-subtext-0 text-xs">{t("settings.gherkinLang")}</span>
              <p className="text-text">{config.output.gherkin_language}</p>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
