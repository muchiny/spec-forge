import { useTranslation } from "react-i18next";
import { useQuery } from "@tanstack/react-query";
import { tauriInvoke } from "@/shared/api/tauri-client";
import { useThemeStore } from "@/shared/hooks/use-theme";
import { useLocaleStore } from "@/shared/stores/use-locale-store";
import { supportedLocales, localeLabels } from "@/shared/config/i18n";
import { Settings, Sun, Moon, Globe, Cpu, Workflow } from "lucide-react";
import { cn } from "@/shared/lib/utils";
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
    <div data-testid="settings-page" className="min-w-0 space-y-6">
      {/* Page header */}
      <div className="flex items-center gap-3">
        <div className="bg-overlay-0/25 flex h-9 w-9 items-center justify-center rounded-lg">
          <Settings className="text-overlay-1 h-5 w-5" />
        </div>
        <div>
          <h2 className="text-text text-xl font-bold">{t("settings.title")}</h2>
          <p className="text-subtext-0 text-sm">{t("settings.subtitle")}</p>
        </div>
      </div>

      {/* Appearance */}
      <div className="bg-mantle rounded-xl border p-5 transition-all duration-200 hover:border-blue/20">
        <div className="mb-4 flex items-center gap-2">
          <Sun className="text-yellow h-4 w-4" />
          <h3 className="text-text font-semibold">
            {t("settings.appearance")}
          </h3>
        </div>

        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <span className="text-subtext-0 text-sm">
              {t("settings.theme")}
            </span>
            <button
              data-testid="settings-theme-toggle"
              onClick={toggleTheme}
              className="bg-surface-0 hover:bg-surface-1 text-text flex items-center gap-2 rounded-lg px-4 py-2.5 text-sm transition-all duration-200"
            >
              {theme === "dark" ? (
                <>
                  <Moon className="text-blue h-4 w-4" />
                  {t("settings.dark")}
                </>
              ) : (
                <>
                  <Sun className="text-yellow h-4 w-4" />
                  {t("settings.light")}
                </>
              )}
            </button>
          </div>

          <div className="border-surface-1 border-t" />

          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <Globe className="text-subtext-0 h-4 w-4" />
              <span className="text-subtext-0 text-sm">
                {t("settings.language")}
              </span>
            </div>
            <div className="bg-crust inline-flex gap-1 rounded-xl p-1">
              {supportedLocales.map((loc) => (
                <button
                  key={loc}
                  onClick={() => setLocale(loc)}
                  className={cn(
                    "rounded-lg px-4 py-2 text-xs font-medium transition-all duration-200",
                    locale === loc
                      ? "bg-blue text-crust shadow-md"
                      : "text-subtext-1 hover:bg-surface-0 hover:text-text",
                  )}
                >
                  {localeLabels[loc]}
                </button>
              ))}
            </div>
          </div>
        </div>
      </div>

      {/* LLM Config */}
      {config && (
        <div className="bg-mantle rounded-xl border p-5 transition-all duration-200 hover:border-blue/20">
          <div className="mb-4 flex items-center gap-2">
            <Cpu className="text-sapphire h-4 w-4" />
            <h3 className="text-text font-semibold">{t("settings.llm")}</h3>
          </div>
          <div className="grid grid-cols-2 gap-x-6 gap-y-4 text-sm lg:grid-cols-3">
            {[
              { label: "settings.provider", value: config.llm.provider },
              { label: "settings.model", value: config.llm.model_name },
              { label: "settings.url", value: config.llm.api_base_url },
              { label: "settings.temperature", value: config.llm.temperature },
              { label: "settings.maxTokens", value: config.llm.max_tokens },
              { label: "settings.contextSize", value: config.llm.context_size },
            ].map((item) => (
              <div key={item.label}>
                <span className="text-subtext-0 text-xs">{t(item.label)}</span>
                <p className="text-text mt-0.5 font-medium">
                  {String(item.value)}
                </p>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Pipeline Config */}
      {config && (
        <div className="bg-mantle rounded-xl border p-5 transition-all duration-200 hover:border-blue/20">
          <div className="mb-4 flex items-center gap-2">
            <Workflow className="text-green h-4 w-4" />
            <h3 className="text-text font-semibold">
              {t("settings.pipeline")}
            </h3>
          </div>
          <div className="grid grid-cols-2 gap-x-6 gap-y-4 text-sm lg:grid-cols-3">
            {[
              {
                label: "settings.defaultLanguage",
                value: config.pipeline.default_language,
              },
              {
                label: "settings.maxRetries",
                value: config.pipeline.max_retries,
              },
              {
                label: "settings.compliance",
                value: config.compliance.profile,
              },
              {
                label: "settings.gherkinLang",
                value: config.output.gherkin_language,
              },
            ].map((item) => (
              <div key={item.label}>
                <span className="text-subtext-0 text-xs">{t(item.label)}</span>
                <p className="text-text mt-0.5 font-medium">
                  {String(item.value)}
                </p>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
