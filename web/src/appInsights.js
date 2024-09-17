import { ApplicationInsights } from '@microsoft/applicationinsights-web'
import { ReactPlugin } from '@microsoft/applicationinsights-react-js'
import { createBrowserHistory } from 'history'

const browserHistory = createBrowserHistory()

export const reactPlugin = new ReactPlugin()

export const appInsights = new ApplicationInsights({
  config: {
    connectionString: `InstrumentationKey=${import.meta.env.VITE_APP_AI_INSTRUMENTATION_KEY};IngestionEndpoint=https://eastus-8.in.applicationinsights.azure.com/;LiveEndpoint=https://eastus.livediagnostics.monitor.azure.com/;ApplicationId=262105ba-2fe1-40c7-b3f3-fcd58d34acbc`,
    enableAutoRouteTracking: true,
    extensions: [reactPlugin],
    extensionConfig: {
      [reactPlugin.identifier]: { history: browserHistory },
    },
  },
})
appInsights.loadAppInsights()
appInsights.addTelemetryInitializer((envelope) => {
  envelope.data = envelope.data || {}
  envelope.data.appVersion = import.meta.env.VITE_APP_VERSION
  envelope.data.commitId = import.meta.env.VITE_APP_COMMIT_ID
})
