import { createPhenotypeConfig } from '@phenotype/docs/config'

const isPagesBuild = process.env.GITHUB_ACTIONS === 'true' || process.env.GITHUB_PAGES === 'true'
const repoName = process.env.GITHUB_REPOSITORY?.split('/')[1] || 'phenotype-shared'
const docsBase = isPagesBuild ? `/${repoName}/` : '/'

export default createPhenotypeConfig({
  title: 'phenotype-shared',
  description: 'Shared Phenotype infrastructure components',
  overrides: {
    lang: 'en-US',
    base: docsBase,
    lastUpdated: true,
    cleanUrls: true,
    markdown: { lineNumbers: true },
    ignoreDeadLinks: true,
    themeConfig: {
      siteTitle: 'phenotype-shared',
      nav: [{ text: 'Guide', link: '/guide/' }],
      sidebar: {
        '/guide/': [
          { text: 'Guide', items: [{ text: 'Getting Started', link: '/guide/' }] }
        ]
      },
      socialLinks: [{ icon: 'github', link: `https://github.com/KooshaPari/${repoName}` }],
      search: { provider: 'local' }
    },
  },
})
