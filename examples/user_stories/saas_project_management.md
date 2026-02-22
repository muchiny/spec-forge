# User Stories - SaaS Project Management Tool "TeamForge"

## Real-time collaborative task board

As a project manager, I want to manage tasks on a drag-and-drop Kanban board that updates in real-time so that my entire team sees changes instantly without refreshing.

- The board supports custom columns (minimum: To Do, In Progress, Done) with a maximum of 10 columns
- Dragging a task between columns updates the status for all connected users within 500ms
- Each task card displays the title, assignee avatar, priority tag and due date
- WIP limits can be configured per column with a visual warning when exceeded
- Tasks can be filtered by assignee, priority, label or due date range
- The board loads within 2 seconds even with 500+ tasks across columns
- Undo is available for the last 5 drag-and-drop actions via Ctrl+Z

## Sprint planning with capacity estimation

As a scrum master, I want to plan sprints with story point estimation and team capacity tracking so that I can commit to realistic deliverables.

- The sprint backlog shows the total story points committed versus team capacity
- Team capacity is calculated from working days minus planned absences per member
- Story points can be assigned via planning poker with real-time voting visible to all participants
- The velocity chart shows the average of the last 5 sprints as reference
- Overcommitted sprints are highlighted in red with the excess percentage
- Stories can be dragged from the product backlog to the sprint backlog
- A sprint cannot be started if it has zero committed stories

## Time tracking and reporting

As a team member, I want to log time spent on tasks and generate reports so that the team can improve estimation accuracy and clients can be billed correctly.

- A timer can be started and stopped directly from the task detail view
- Manual time entries are supported with date, duration and description fields
- The weekly timesheet view shows logged hours per day with a total
- Overtime beyond 8 hours per day is highlighted in amber
- Reports can be generated per project, per team member or per client
- Export to CSV and PDF is available for all reports
- Billable versus non-billable hours are tracked separately with distinct totals
- Time entries older than 7 days require manager approval to modify

## Automated CI/CD integration

As a developer, I want to see the build and deployment status of my branches directly in the task view so that I can track the progress from code to production.

- GitHub, GitLab and Bitbucket integrations are supported via OAuth
- Each task shows the linked branch, latest commit message and CI pipeline status
- Build failures trigger a notification to the task assignee and the reviewer
- The deployment timeline shows which environments (staging, production) have which version
- Pull request links are automatically attached to tasks when the branch name contains the task ID
- Code review status (approved, changes requested, pending) is visible on the task card
- Merging a PR automatically moves the task to the next column if auto-transition is enabled

## Role-based access control

As an organization admin, I want to define custom roles with granular permissions so that team members only access what they need.

- Default roles include Admin, Project Manager, Member and Viewer
- Custom roles can be created with permissions selected from a checklist
- Permissions cover: create/edit/delete projects, manage members, view reports, manage billing, access admin panel
- Role changes take effect immediately without requiring the user to re-login
- The audit log records all permission changes with timestamp, actor and affected user
- A user can have different roles in different projects within the same organization
- At least one Admin must remain in each organization to prevent lockout

## Client portal with limited visibility

As a client stakeholder, I want to access a simplified view of my project progress so that I can follow milestones and provide feedback without seeing internal discussions.

- The client portal shows only tasks tagged as "client-visible"
- Milestones with progress bars and due dates are displayed on a timeline
- Clients can add comments on visible tasks that appear in a separate "client feedback" thread
- File attachments shared by the team are only visible to clients if explicitly marked as shared
- The portal uses the organization's branding (logo, primary color)
- No internal comments, time logs or story points are visible to clients
- Access is controlled via invitation link with email verification

## Advanced search and filtering

As a power user, I want to search across all projects with advanced filters and save my queries so that I can quickly find any task or discussion.

- Full-text search covers task titles, descriptions, comments and attachments
- Filters support AND/OR logic with nested conditions
- Saved searches can be pinned to the sidebar for quick access
- Search results display a snippet of the matching text with the keyword highlighted
- Results are paginated with 25 items per page and sorted by relevance by default
- Filters include: status, assignee, project, date range, priority, label, has attachments, has comments
- Search response time is under 1 second for organizations with up to 100,000 tasks

## Webhook and API integration

As a technical lead, I want to configure webhooks and access a REST API so that I can integrate the tool with our internal systems and automation workflows.

- Webhooks can be configured for events: task created, status changed, comment added, sprint started, sprint completed
- The webhook payload follows a documented JSON schema with event type, timestamp and full object data
- API rate limits are set at 1000 requests per minute per API key
- API keys can be scoped to specific projects with read-only or read-write access
- The API supports pagination, filtering and sorting on all list endpoints
- A Swagger/OpenAPI documentation page is auto-generated and always up to date
- Webhook delivery failures are retried 3 times with exponential backoff and logged in the admin panel
